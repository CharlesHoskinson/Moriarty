"""Prospective positive input identity tests. UNRUN; no admission negatives."""
import copy
import json
from pathlib import Path
import sys
import unittest
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"source"))
import positive_identity_port as p
INPUTS=Path(__file__).resolve().parents[1]/"inputs"
def source(name):
    return json.loads((INPUTS/(name+".json")).read_text())
class PositiveIdentityTests(unittest.TestCase):
    def test_complete_raw_identity(self):
        for name in ("map-queue","mirror-arbor"):
            raw=source(name); saved=copy.deepcopy(raw); out=p.port_positive(raw)
            self.assertEqual(raw,saved)
            self.assertEqual(out["source_document"],saved)
            self.assertEqual(p.source_roundtrip(out),saved)
            self.assertEqual(set(out["source_document"]["program_view"]),set(p.PROGRAM_FIELDS))
    def test_runtime_global_ids_and_order(self):
        out=p.port_positive(source("map-queue"))
        ds=out["ng_program"]["declarations"]
        self.assertEqual([x["type_id"] for x in ds],[2,3,4,5])
        self.assertEqual([x["ctor_id"] for x in ds[-2]["ctors"]],[4,5])
        self.assertEqual([x["source_order"] for x in ds[-2]["ctors"]],[0,1])
        self.assertEqual(ds[-2]["ctors"][1]["field_types"],[0,4])
        self.assertEqual(out["ctor_rows"][-1],{"type_id":5,"ctor_id":6,"name":"Wrap","fields":["z_items","a_marker"]})
    def test_generic_arguments_are_not_sorted(self):
        for name,expected in (("map-queue",[4,1]),("mirror-arbor",[4,0])):
            out=p.port_positive(source(name)); last=out["type_rows"][-1]
            self.assertEqual([x["runtime_type_id"] for x in last["type"]["arguments"]],expected)
    def test_equal_typed_fields_keep_declaration_order(self):
        out=p.port_positive(source("mirror-arbor"))
        self.assertEqual(out["ctor_rows"][5]["fields"],["z_left","a_right"])
        self.assertEqual(out["ng_program"]["declarations"][-2]["ctors"][1]["field_types"],[4,4])
    def test_empty_slice_and_some_zero_preserved(self):
        raw=source("map-queue"); out=p.port_positive(raw); original=p.source_roundtrip(out)["program_view"]
        self.assertEqual(original["constructors"][4]["field_start"],"5")
        self.assertEqual(original["constructors"][4]["field_len"],"0")
        self.assertEqual(original["types"][4]["decl"],"0")
        self.assertIsNone(original["types"][0]["decl"])
        self.assertEqual(original["expressions"][6]["Match"]["plan"],"0")
        self.assertEqual(len(original["constants"]),6)
    def test_not_uses_existing_semantics(self):
        out=p.port_positive(source("mirror-arbor"))
        self.assertEqual(out["ng_program"]["expressions"][12],{"id":12,"occurrence":12,"kind":{"Not":{"operand":13}}})
        self.assertEqual(out["ng_program"]["functions"][0]["name_id"],8)
    def test_whole_source_ast_and_query_are_data(self):
        raw=source("map-queue"); out=p.port_positive(raw); query=p.query_ast(out,512)
        self.assertEqual(query["sort"],"NIObservation")
        self.assertEqual(query["constructor"],"niRun")
        self.assertEqual(p.json_from_ast(query["arguments"][0]),raw)
        self.assertEqual(query["arguments"][-1],{"sort":"Int","literal":512})
    def test_names_are_input_derived(self):
        raw=source("map-queue"); raw["program_view"]["names"][4]="WrappedAgain"
        out=p.port_positive(raw)
        self.assertEqual(out["ctor_rows"][-1]["name"],"WrappedAgain")
        self.assertEqual(out["ctor_rows"][-1]["ctor_id"],6)
if __name__=="__main__": unittest.main()
