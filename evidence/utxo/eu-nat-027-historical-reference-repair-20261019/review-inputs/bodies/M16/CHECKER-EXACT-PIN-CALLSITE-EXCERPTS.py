    def pin(self, value, where, issues):
        obj(value, ("path", "sha256"), where)
        string(value["path"], f"{where}.path")
        require(type(value["sha256"]) is str and
                re.fullmatch(r"[0-9a-f]{64}", value["sha256"]) is not None,
                f"{where}.sha256: expected full lowercase SHA-256")
        try:
            raw = self.read(value["path"])
        except FileNotFoundError:
            issues.append(f"{where}: missing file {value['path']}")
            return None
        except OSError as exc:
            raise Malformed(f"{where}: unsafe or unreadable path {value['path']}: {exc.strerror}") from exc
        if hashlib.sha256(raw).hexdigest() != value["sha256"]:
            issues.append(f"{where}: stale pin {value['path']}")
        return raw

# EXACT SECOND EXCERPT: original lines225–238
        refs = [(ref, "validation evidence") for ref in array(row["validation"]["evidenceRefs"], "validation refs")]
        reviews = array(row["acceptance"]["reviewRefs"], "review refs")
        refs += [(ref, "review") for ref in reviews]
        pins = array(row["sourcePins"], "source pins")
        refs += [(ref, "source pin") for ref in pins]
        files.pin(row["source"], f"{key}: source", local)
        for record in array(row["scenarioRecords"], f"{key}.scenarioRecords"):
            obj(record, ("name", "status", "expectation", "evidenceRefs"), "scenario record")
            for field in ("name", "status", "expectation"):
                string(record[field], f"scenario.{field}")
            record_names.append(record["name"])
            refs += [(ref, f"scenario {record['name']} evidence") for ref in array(record["evidenceRefs"], "scenario refs")]
        for ref, label in refs:
            files.pin(ref, f"{key}: {label}", local)
