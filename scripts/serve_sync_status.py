#!/usr/bin/env python3
"""Serve the latest verified harness state from memory over a Unix socket."""

import argparse
import asyncio
import contextlib
import json
import os
import time
from pathlib import Path

if __package__:
    from scripts import run_sync_assessment as assessment
else:
    import run_sync_assessment as assessment


class StatusState:
    def __init__(self) -> None:
        self.result = None
        self.error = None
        self.refreshed_at = 0.0
        self.refreshing = False
        self.sequence = 0

    def view(self) -> dict[str, object]:
        ready = self.result is not None
        return {
            "service_state": "ready" if ready else "error" if self.error else "verifying",
            "sequence": self.sequence,
            "verified_at_unix_ms": round(self.refreshed_at * 1000) if ready else None,
            "age_ms": round((time.time() - self.refreshed_at) * 1000) if ready else None,
            "refreshing": self.refreshing,
            "result": self.result,
            "error": self.error,
        }

    async def refresh(self, timeout: int) -> None:
        if self.refreshing:
            return
        self.refreshing = True
        try:
            self.result = await asyncio.to_thread(assessment.assess, timeout)
            self.error = None
            self.refreshed_at = time.time()
            self.sequence += 1
        except Exception as error:  # preserve the last verified result
            self.error = str(error)
        finally:
            self.refreshing = False


async def respond(state: StatusState, _reader: asyncio.StreamReader, writer: asyncio.StreamWriter) -> None:
    writer.write(json.dumps(state.view(), separators=(",", ":")).encode() + b"\n")
    await writer.drain()
    writer.close()
    await writer.wait_closed()


async def refresh_loop(state: StatusState, interval: float, timeout: int) -> None:
    while True:
        await state.refresh(timeout)
        await asyncio.sleep(interval)


async def serve(socket_path: Path, interval: float, timeout: int) -> None:
    if socket_path.exists():
        raise RuntimeError(f"socket already exists: {socket_path}")
    state = StatusState()
    server = await asyncio.start_unix_server(lambda r, w: respond(state, r, w), path=socket_path)
    os.chmod(socket_path, 0o600)
    refresher = asyncio.create_task(refresh_loop(state, interval, timeout))
    try:
        async with server:
            await server.serve_forever()
    finally:
        refresher.cancel()
        with contextlib.suppress(asyncio.CancelledError):
            await refresher
        socket_path.unlink(missing_ok=True)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--socket", type=Path, default=Path(f"/tmp/harness-sync-status-{os.getuid()}.sock"))
    parser.add_argument("--interval", type=float, default=5.0)
    parser.add_argument("--timeout", type=int, default=120)
    arguments = parser.parse_args()
    try:
        asyncio.run(serve(arguments.socket, arguments.interval, arguments.timeout))
    except (KeyboardInterrupt, RuntimeError) as error:
        if not isinstance(error, KeyboardInterrupt):
            print(json.dumps({"status": "blocked", "error": str(error)}))
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
