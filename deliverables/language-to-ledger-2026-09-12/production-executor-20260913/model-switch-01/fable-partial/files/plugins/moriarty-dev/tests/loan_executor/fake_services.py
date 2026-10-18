"""Explicit fake user-systemd and Docker transports for loan executor tests.

One state machine serves two shapes: an in-process ``run_command`` for the
fault-table tests and thin fake executables (systemctl, systemd-run, docker)
for the real subprocess smoke. State lives in files under a state directory
so the subprocess shape survives across processes; the executable shape is
the same class driven from ``__main__``. Nothing here touches a live user
manager, Docker daemon, wallet, network or canonical store.
"""
import json
import os
from pathlib import Path
import secrets
import sys
import time

NOT_FOUND = {
    "LoadState": "not-found", "ActiveState": "inactive", "SubState": "dead", "MainPID": "0",
    "ControlGroup": "", "InvocationID": "", "Result": "success", "ExecMainCode": "0", "ExecMainStatus": "0",
    "Type": "simple", "RemainAfterExit": "no", "Transient": "no", "ActiveEnterTimestampMonotonic": "0",
    "NextElapseUSecMonotonic": "0", "Unit": "",
}
NEVER = "0001-01-01T00:00:00Z"


class Result(object):
    def __init__(self, argv, returncode, stdout="", stderr="", timed_out=False):
        self.argv = list(argv)
        self.returncode = returncode
        self.stdout = stdout
        self.stderr = stderr
        self.timed_out = timed_out


class FakeServices(object):
    """Scripted systemctl/systemd-run/docker behavior keyed by a state directory.

    ``script`` (state/script.json) controls the financial unit outcome:
      runningPolls: number of running observations before the terminal one
      terminal: {"kind": "exit"|"signal"|"unloaded"|"transitional"|"malformed"|"changed-invocation", "code": int}
      stopReturnCode / killReturnCode / dockerStartReturnCode / timerArmReturnCode / timerCancelReturnCode
      showReturnCode / showTimeout: show failures
      dockerStartedAt: StartedAt reported after start
      replacementAfterStart: report a different StartedAt on later inspects (replacement generation)
    """

    def __init__(self, state_dir):
        self.state = Path(state_dir)
        self.state.mkdir(parents=True, exist_ok=True)

    # --- state helpers ----------------------------------------------------------
    def _load(self, name, default):
        path = self.state / name
        if not path.is_file():
            return default
        return json.loads(path.read_text())

    def _save(self, name, value):
        (self.state / name).write_text(json.dumps(value, indent=1))

    def script(self):
        return self._load("script.json", {})

    def now_ns(self):
        """Fake manager clock: real monotonic plus the harness's injected offset."""
        offset = self._load("clock-offset.json", {"seconds": 0}).get("seconds", 0)
        return time.monotonic_ns() + int(float(offset) * 1_000_000_000)

    def log(self, argv):
        with (self.state / "calls.log").open("a") as handle:
            handle.write(json.dumps(list(argv)) + "\n")

    def calls(self):
        path = self.state / "calls.log"
        if not path.is_file():
            return []
        return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]

    def seed_container(self, config):
        self._save("container.json", config)

    def set_unit(self, unit, fields):
        self._save("unit-" + unit + ".json", fields)

    def unit(self, unit):
        return self._load("unit-" + unit + ".json", None)

    # --- dispatch --------------------------------------------------------------------
    def handle(self, argv):
        argv = list(argv)
        self.log(argv)
        if argv and Path(argv[0]).name == "timeout":
            # The bound wrapper is part of the retained argv; the fake honours the
            # inner command and records the bound.
            inner = argv[3:]
            return self._dispatch(inner, argv)
        return self._dispatch(argv, argv)

    def _dispatch(self, inner, full):
        if not inner:
            return Result(full, 2, "", "empty argv")
        name = Path(inner[0]).name
        if name == "systemctl":
            return self._systemctl(inner, full)
        if name == "systemd-run":
            return self._systemd_run(inner, full)
        if name == "docker":
            return self._docker(inner, full)
        return Result(full, 127, "", "unknown fake executable " + name)

    # --- systemctl -----------------------------------------------------------------
    def _systemctl(self, inner, full):
        script = self.script()
        if "--user" not in inner:
            return Result(full, 1, "", "fake systemctl requires --user")
        verb = inner[2]
        if verb == "show":
            unit = inner[3]
            props = [p for p in inner[4:] if p.startswith("--property=")]
            if not props:
                return Result(full, 1, "", "property required")
            terminal = "Result" in props[0]  # the collector's terminal observation
            if terminal and script.get("showReturnCode") and unit.endswith(".service") and self.unit(unit):
                return Result(full, script["showReturnCode"], "", "fake show failure")
            if terminal and script.get("showTimeout") and unit.endswith(".service") and self.unit(unit):
                return Result(full, None, "", "", timed_out=True)
            if script.get("identityShowTimeout") and unit.endswith(".service") and self.unit(unit):
                return Result(full, None, "", "", timed_out=True)
            fields = self._observe(unit, consume=terminal)
            if terminal and script.get("malformedShow") and unit.endswith(".service") and fields.get("LoadState") == "loaded" \
                    and fields.get("SubState") != "running":
                return Result(full, 0, "FINANCIAL_COMPLETE\n", "")
            wanted = props[0][len("--property="):].split(",")
            return Result(full, 0, "".join("%s=%s\n" % (k, fields.get(k, "")) for k in wanted), "")
        if verb == "stop":
            unit = inner[3]
            rc = script.get("stopReturnCode", 0)
            if script.get("stopTimeout"):
                return Result(full, None, "", "", timed_out=True)
            if rc == 0:
                if unit.endswith(".timer"):
                    (self.state / ("unit-" + unit + ".json")).unlink(missing_ok=True)
                else:
                    (self.state / ("unit-" + unit + ".json")).unlink(missing_ok=True)
            return Result(full, rc, "", "" if rc == 0 else "fake stop failure")
        if verb == "kill":
            rc = script.get("killReturnCode", 0)
            unit = inner[-1]
            current = self.unit(unit)
            if rc == 0 and current and current.get("SubState") == "running":
                if script.get("replaceUnitAfterKill"):
                    # A replacement invocation appears under the same name after the kill.
                    current.update({"InvocationID": secrets.token_hex(16), "MainPID": "5151"})
                else:
                    current.update({"ActiveState": "failed", "SubState": "failed", "MainPID": "0", "Result": "signal",
                                    "ExecMainCode": "2", "ExecMainStatus": "9", "ControlGroup": ""})
                self.set_unit(unit, current)
            return Result(full, rc, "", "")
        return Result(full, 1, "", "unsupported fake systemctl verb " + verb)

    def _observe(self, unit, consume=True):
        current = self.unit(unit)
        if current is None:
            return dict(NOT_FOUND)
        if consume and unit.endswith(".service") and current.get("SubState") == "running" and "polls" in current:
            current["polls"] -= 1
            if current["polls"] < 0:
                current = self._terminal(current)
            self.set_unit(unit, current)
        return current

    def _terminal(self, current):
        terminal = self.script().get("terminal", {"kind": "exit", "code": 0})
        kind = terminal.get("kind", "exit")
        code = terminal.get("code", 0)
        current = dict(current)
        current.pop("polls", None)
        if kind == "exit":
            current.update({"ActiveState": "active" if code == 0 else "failed",
                            "SubState": "exited" if code == 0 else "failed", "MainPID": "0",
                            "Result": "success" if code == 0 else "exit-code", "ExecMainCode": "1",
                            "ExecMainStatus": str(code)})
        elif kind == "signal":
            current.update({"ActiveState": "failed", "SubState": "failed", "MainPID": "0", "Result": "signal",
                            "ExecMainCode": "2", "ExecMainStatus": str(code)})
        elif kind == "unloaded":
            current = dict(NOT_FOUND)
        elif kind == "transitional":
            current.update({"ActiveState": "deactivating", "SubState": "stop-sigterm"})
        elif kind == "changed-invocation":
            current.update({"ActiveState": "active", "SubState": "exited", "MainPID": "0", "Result": "success",
                            "ExecMainCode": "1", "ExecMainStatus": "0", "InvocationID": secrets.token_hex(16)})
        elif kind == "default-identity":
            current.update({"ActiveState": "active", "SubState": "exited", "MainPID": "0", "Result": "success",
                            "ExecMainCode": "0", "ExecMainStatus": "0"})
        elif kind == "malformed":
            current.update({"ActiveState": "active", "SubState": "exited", "MainPID": "0", "Result": "success",
                            "ExecMainCode": "1", "ExecMainStatus": "zero"})
        elif kind == "exit-wrong-result":
            current.update({"ActiveState": "active", "SubState": "exited", "MainPID": "0", "Result": "exit-code",
                            "ExecMainCode": "1", "ExecMainStatus": "0"})
        return current

    # --- systemd-run ------------------------------------------------------------------
    def _systemd_run(self, inner, full):
        script = self.script()
        unit_arg = next((a for a in inner if a.startswith("--unit=")), None)
        if unit_arg is None:
            return Result(full, 1, "", "unit required")
        name = unit_arg[len("--unit="):]
        if any(a.startswith("--on-active=") for a in inner):
            rc = script.get("timerArmReturnCode", 0)
            if rc != 0:
                return Result(full, rc, "", "fake timer arm failure")
            delay = next(a for a in inner if a.startswith("--on-active="))[len("--on-active="):]
            next_elapse_us = (self.now_ns() + int(delay.rstrip("s")) * 1_000_000_000) // 1000 \
                + int(script.get("timerNextElapseOffsetSeconds", 0)) * 1_000_000
            self.set_unit(name + ".timer", {"LoadState": "loaded", "ActiveState": "active", "SubState": "waiting",
                                            "NextElapseUSecMonotonic": str(next_elapse_us),
                                            "Unit": script.get("timerUnitName") or name + ".service"})
            self._save("timer-command.json", inner)
            return Result(full, 0, "", "Running timer as unit: %s.timer\n" % name)
        if self.unit(name) is not None and self.unit(name).get("LoadState") == "loaded":
            return Result(full, 1, "", "Unit %s already exists" % name)
        rc = script.get("unitLaunchReturnCode", 0)
        if rc != 0:
            return Result(full, rc, "", "fake launch failure")
        activation_us = self.now_ns() // 1000 + int(script.get("activationDelaySeconds", 0)) * 1_000_000
        fields = {
            "LoadState": "loaded", "ActiveState": "active", "SubState": "running", "MainPID": "4242",
            "ControlGroup": "/user.slice/user-1000.slice/user@1000.service/app.slice/" + name,
            "InvocationID": script.get("invocationId") or secrets.token_hex(16),
            "ActiveEnterTimestampMonotonic": str(activation_us), "Result": "success",
            "ExecMainCode": "0", "ExecMainStatus": "0", "Type": "exec", "RemainAfterExit": "yes",
            "Transient": "yes", "polls": int(script.get("runningPolls", 1)),
        }
        if script.get("startupMissingInvocation"):
            fields["InvocationID"] = "0" * 32
        if script.get("startupMissingTimestamp"):
            fields["ActiveEnterTimestampMonotonic"] = ""
        self.set_unit(name, fields)
        self._save("launch-command.json", inner)
        return Result(full, 0, "", "Running as unit: %s\n" % name)

    # --- docker -------------------------------------------------------------------------
    def _docker(self, inner, full):
        script = self.script()
        config = self._load("container.json", None)
        verb = inner[1] if len(inner) > 1 else ""
        container_id = inner[-1]
        if config is None or config.get("Id") != container_id:
            return Result(full, 1, "", "Error: No such object: " + container_id)
        if verb == "inspect":
            fmt = inner[inner.index("--format") + 1] if "--format" in inner else "{{json .}}"
            state = config["State"]
            if fmt == "{{json .}}":
                return Result(full, 0, json.dumps(config) + "\n", "")
            started = state["StartedAt"]
            if script.get("replacementAfterStart") and state.get("Running") and config.get("inspects", 0) >= 1:
                started = "2099-01-01T00:00:00.000000000Z"
            config["inspects"] = config.get("inspects", 0) + 1
            self._save("container.json", config)
            return Result(full, 0, "%s|%s|%s|%s\n" % (config["Id"], started, "true" if state["Running"] else "false",
                                                       state["Status"]), "")
        if verb == "start":
            rc = script.get("dockerStartReturnCode", 0)
            if rc != 0:
                return Result(full, rc, "", "fake docker start failure")
            config["State"] = {"Status": "running", "Running": True,
                               "StartedAt": script.get("dockerStartedAt", "2026-09-13T20:00:00.123456789Z")}
            self._save("container.json", config)
            return Result(full, 0, container_id + "\n", "")
        if verb == "kill":
            rc = script.get("dockerKillReturnCode", 0)
            if rc == 0 and config["State"]["Running"]:
                config["State"] = dict(config["State"], Running=False, Status="exited")
                self._save("container.json", config)
            return Result(full, rc, container_id + "\n" if rc == 0 else "", "" if rc == 0 else "fake kill failure")
        return Result(full, 1, "", "unsupported fake docker verb " + verb)


def container_config(container_id, image_digest, memory, wrapper_host, control_host, wrapper_dest, control_dest,
                     control_path, args=None):
    return {
        "Id": container_id, "Image": image_digest,
        "State": {"Status": "created", "Running": False, "StartedAt": NEVER},
        "Path": wrapper_dest,
        "Args": list(args) if args else [control_path, "--", "/usr/local/bin/midnight-proof-server", "--port", "6300"],
        "HostConfig": {"RestartPolicy": {"Name": "no"}, "PidMode": "", "Privileged": False, "CapAdd": None,
                       "Memory": memory},
        "Mounts": [{"Source": wrapper_host, "Destination": wrapper_dest, "RW": False},
                   {"Source": control_host, "Destination": control_dest, "RW": False}],
    }


def run_command_for(services):
    def run_command(argv, timeout_seconds=30):
        return services.handle(argv)
    return run_command


def write_fake_executables(directory, state_dir):
    """Thin executables that drive the same state machine from a subprocess."""
    directory = Path(directory)
    directory.mkdir(parents=True, exist_ok=True)
    table = {}
    for name in ("systemctl", "systemd-run", "docker"):
        target = directory / name
        target.write_text(
            "#!%s\nimport sys\nsys.path.insert(0, %r)\nimport fake_services\n"
            "result = fake_services.FakeServices(%r).handle([%r, *sys.argv[1:]])\n"
            "sys.stdout.write(result.stdout)\nsys.stderr.write(result.stderr)\n"
            "raise SystemExit(124 if result.timed_out else result.returncode)\n"
            % (sys.executable, str(Path(__file__).resolve().parent), str(state_dir), "/usr/bin/" + name)
        )
        target.chmod(0o700)
        table["/usr/bin/" + name] = str(target)
    return table


if __name__ == "__main__":
    raise SystemExit(2)
