"""Actual N01 fuel-1 capture fixtures and synthetic hostile mutations for the fixed 05 adapter.

Actual fixtures are byte-exact base64 literals copied from ACTUAL-FIXTURE-BYTES.json (sha256
21d24dd656ed132f061bde507f2db5b8a947da44966d2df95e5649f71e6c7ad7), the transport of outputs
captured once in attempt-engine-N01-01 (receipt.json sha256
d6123acfdfd16128d533701dae3316bf166c08e5ae393fccbb958657f38d02cc). RECORDED sizes and hashes
are transcribed from that receipt's postflight tree. Receipts are rebuilt from its recorded
stage commands, return codes and artifact hashes: declarations only, not authenticated.
INVENTORY is a verbatim subset of COMPILED-MAIN-RULE-SPANS-01.json (sha256
9e77ddfe74acaa1e2deccaa1fcde70794494dd3583362b3f9a6f58473d9e941c).

Hostile cases mutate fresh copies of actual fixtures through replaced(), relogged() and
reprinted(); they are synthetic, not K runs. Asserted values are read from the fixture bytes
themselves: no oracle, expectation, independent comparison data or K match is used.
"""
import base64
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import time
import unittest


MODULE = Path(__file__).with_name('k_capture_observations.py')
SPEC = importlib.util.spec_from_file_location('k_capture_observations', MODULE)
k = None
if MODULE.exists():
    k = importlib.util.module_from_spec(SPEC)
    SPEC.loader.exec_module(k)

ROOT = '/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01'
STAGES = ROOT + '/attempt-engine-N01-01/stages'
KOMPILED = ROOT + '/attempt-compile-01/main-kompiled'
SOURCE = ROOT + '/compile-input/models/sherlock/k/main.k'
KORE_EXEC = '/nix/store/qb0m8rd0c3b90dfbz8fwlczd3kc3828z-kore-exec/bin/.kore-exec-wrapped'
KPRINT = '/nix/store/p7hjh21dj4bc1s6kfcdwk45knp5n4cc0-llvm-backend-0.1.140/bin/kprint'
EMPTY = 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
MAIN_K_SHA256 = 'b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41'
FIXED_SOURCES = {
    'k_capture_observations.py': (
        41183, '5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1'),
    'k_observations.py': (7686, '540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07'),
    'k_terms.py': (5666, 'c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca'),
}
# inputs/N01-fuel-001.ui, the separately retained run input.
RUN_INPUT = 'uiRun(uiNoFuns(), uiLiteral(uiU(7)), 1, 8)\n'
RUN_INPUT_RECORD = (43, 'e5fe7b26e9565fdafdc97da0fb25fb81c6171afd69e99eaa6dfb38b5409a2e0f')
ENTRY = 'uiLiteral(uiU(7))'
CELLS = ('k', 'env', 'stack', 'depth', 'stackLimit', 'code', 'fuel', 'initialFuel', 'out')

LOG_03 = 'a29yZS1leGVjOiBXYXJuaW5nIChXYXJuRGVwdGhMaW1pdEV4Y2VlZGVkKToKICAgIFRoZSBkZXB0aCBsaW1pdCAwIHdhcyBleGNlZWRlZC4Ka29yZS1leGVjOiBJbmZvIChJbmZvRXhlY0RlcHRoKToKICAgIGV4ZWN1dGlvbiBjb21wbGV0ZTogZXhlYyBkZXB0aDogMAo='
KORE_03 = 'TGJsaW5pdEdlbmVyYXRlZFRvcENlbGx7fSgKICAgIC8qIEludGVybmFsTWFwOiAqLyAvKiBjb25jcmV0ZSBlbGVtZW50OiAqLyBMYmwnVW5kc1BpcGUnLSctR1QtVW5kcyd7fSgKICAgICAgICAvKiBJbmo6ICovIGluantTb3J0S0NvbmZpZ1Zhcnt9LCBTb3J0S0l0ZW17fX0oCiAgICAgICAgICAgIFxkdntTb3J0S0NvbmZpZ1Zhcnt9fSgiJFBHTSIpCiAgICAgICAgKSwKICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlSdW57fSwgU29ydEtJdGVte319KAogICAgICAgICAgICBMYmx1aVJ1bidMUGFyVW5kc0NvbW1VbmRzQ29tbVVuZHNDb21tVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJUnVuJ1VuZHMnVUlGdW5zJ1VuZHMnVUlFeHByJ1VuZHMnSW50J1VuZHMnSW50e30oCiAgICAgICAgICAgICAgICBMYmx1aU5vRnVucydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlGdW5ze30oKSwKICAgICAgICAgICAgICAgIExibHVpTGl0ZXJhbCdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJRXhwcidVbmRzJ1VJVmFsdWV7fSgKICAgICAgICAgICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlVSW50e30sIFNvcnRVSVZhbHVle319KAogICAgICAgICAgICAgICAgICAgICAgICBMYmx1aVUnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSVVJbnQnVW5kcydJbnR7fSgKICAgICAgICAgICAgICAgICAgICAgICAgICAgIFxkdntTb3J0SW50e319KCI3IikKICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiMSIpLAogICAgICAgICAgICAgICAgXGR2e1NvcnRJbnR7fX0oIjgiKQogICAgICAgICAgICApCiAgICAgICAgKQogICAgKQop'
LOG_04 = 'a29yZS1leGVjOiBEZWJ1ZyAoRGVidWdBcHBsaWVkUmV3cml0ZVJ1bGVzKToKICAgIFRoZSBydWxlcyBhdCBmb2xsb3dpbmcgbG9jYXRpb25zIHdlcmUgYXBwbGllZDoKICAgIC9ob21lL2NoYXJsL01vcmlhcnR5Ly5tb3JpYXJ0eS1kZXYvcnVudGltZS9zaGVybG9jay1wYXJhbGxlbC0yMDI2MTAxNi9rLWFjdHVhbC0wMS9jb21waWxlLWlucHV0L21vZGVscy9zaGVybG9jay9rL21haW4uazo3NDo4LTc3OjQxCiAgICBPbiBjb25maWd1cmF0aW9uOgogICAgICBcYW5ke1NvcnRHZW5lcmF0ZWRUb3BDZWxse319KAogICAgICAgICAgLyogdGVybTogKi8KICAgICAgICAgIExibCctTFQtJ2dlbmVyYXRlZFRvcCctR1QtJ3t9KAogICAgICAgICAgICAgIExibCctTFQtJ2snLUdULSd7fSgKICAgICAgICAgICAgICAgICAga3NlcXt9KAogICAgICAgICAgICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJUnVue30sIFNvcnRLSXRlbXt9fSgKICAgICAgICAgICAgICAgICAgICAgICAgICBMYmx1aVJ1bidMUGFyVW5kc0NvbW1VbmRzQ29tbVVuZHNDb21tVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJUnVuJ1VuZHMnVUlGdW5zJ1VuZHMnVUlFeHByJ1VuZHMnSW50J1VuZHMnSW50e30oCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIExibHVpTm9GdW5zJ0xQYXJSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSUZ1bnN7fSgpLAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICBMYmx1aUxpdGVyYWwnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSUV4cHInVW5kcydVSVZhbHVle30oCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlVSW50e30sIFNvcnRVSVZhbHVle319KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIExibHVpVSdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJVUludCdVbmRzJ0ludHt9KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiNyIpCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICAgICAgICApLAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiMSIpLAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiOCIpCiAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgKSwKICAgICAgICAgICAgICAgICAgICAgIGRvdGt7fSgpCiAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2VudictR1QtJ3t9KC8qIEludGVybmFsTWFwOiAqLyBMYmwnU3RvcCdNYXB7fSgpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdzdGFjayctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aU5vRnJhbWVzJ0xQYXJSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJRnJhbWVze30oKQogICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgTGJsJy1MVC0nZGVwdGgnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdzdGFja0xpbWl0Jy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nY29kZSctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aU5vRnVucydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlGdW5ze30oKQogICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgTGJsJy1MVC0nZnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIwIikpLAogICAgICAgICAgICAgIExibCctTFQtJ2luaXRpYWxGdWVsJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nb3V0Jy1HVC0ne30oCiAgICAgICAgICAgICAgICAgIExibHVpUGVuZGluZydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSU91dHB1dHt9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2dlbmVyYXRlZENvdW50ZXInLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKQogICAgICAgICAgKSwKICAgICAgXGFuZHtTb3J0R2VuZXJhdGVkVG9wQ2VsbHt9fSgKICAgICAgICAgIC8qIHByZWRpY2F0ZTogKi8KICAgICAgICAgIFx0b3B7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oKSwKICAgICAgICAgIC8qIHN1YnN0aXR1dGlvbjogKi8KICAgICAgICAgIFx0b3B7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oKQogICAgICApKQprb3JlLWV4ZWM6IFdhcm5pbmcgKFdhcm5EZXB0aExpbWl0RXhjZWVkZWQpOgogICAgVGhlIGRlcHRoIGxpbWl0IDEgd2FzIGV4Y2VlZGVkLgprb3JlLWV4ZWM6IEluZm8gKEluZm9FeGVjRGVwdGgpOgogICAgZXhlY3V0aW9uIGNvbXBsZXRlOiBleGVjIGRlcHRoOiAxCg=='
KORE_04 = 'TGJsJy1MVC0nZ2VuZXJhdGVkVG9wJy1HVC0ne30oCiAgICBMYmwnLUxULSdrJy1HVC0ne30oCiAgICAgICAga3NlcXt9KAogICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlDb250cm9se30sIFNvcnRLSXRlbXt9fSgKICAgICAgICAgICAgICAgIExibHVpRXZhbCdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJJ1VuZHMnVUlDb250cm9sJ1VuZHMnVUlFeHBye30oCiAgICAgICAgICAgICAgICAgICAgTGJsdWlMaXRlcmFsJ0xQYXJVbmRzUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlFeHByJ1VuZHMnVUlWYWx1ZXt9KAogICAgICAgICAgICAgICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlVSW50e30sIFNvcnRVSVZhbHVle319KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgTGJsdWlVJ0xQYXJVbmRzUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlVSW50J1VuZHMnSW50e30oCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgXGR2e1NvcnRJbnR7fX0oIjciKQogICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICApCiAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgKQogICAgICAgICAgICApLAogICAgICAgICAgICBkb3Rre30oKQogICAgICAgICkKICAgICksCiAgICBMYmwnLUxULSdlbnYnLUdULSd7fSgvKiBJbnRlcm5hbE1hcDogKi8gTGJsJ1N0b3AnTWFwe30oKSksCiAgICBMYmwnLUxULSdzdGFjayctR1QtJ3t9KAogICAgICAgIExibHVpTm9GcmFtZXMnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJJ1VuZHMnVUlGcmFtZXN7fSgpCiAgICApLAogICAgTGJsJy1MVC0nZGVwdGgnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKSwKICAgIExibCctTFQtJ3N0YWNrTGltaXQnLUdULSd7fShcZHZ7U29ydEludHt9fSgiOCIpKSwKICAgIExibCctTFQtJ2NvZGUnLUdULSd7fSgKICAgICAgICBMYmx1aU5vRnVucydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlGdW5ze30oKQogICAgKSwKICAgIExibCctTFQtJ2Z1ZWwnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMSIpKSwKICAgIExibCctTFQtJ2luaXRpYWxGdWVsJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjEiKSksCiAgICBMYmwnLUxULSdvdXQnLUdULSd7fSgKICAgICAgICBMYmx1aVBlbmRpbmcnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJJ1VuZHMnVUlPdXRwdXR7fSgpCiAgICApLAogICAgTGJsJy1MVC0nZ2VuZXJhdGVkQ291bnRlcictR1QtJ3t9KFxkdntTb3J0SW50e319KCIwIikpCik='
LOG_05 = 'a29yZS1leGVjOiBEZWJ1ZyAoRGVidWdBcHBsaWVkUmV3cml0ZVJ1bGVzKToKICAgIFRoZSBydWxlcyBhdCBmb2xsb3dpbmcgbG9jYXRpb25zIHdlcmUgYXBwbGllZDoKICAgIC9ob21lL2NoYXJsL01vcmlhcnR5Ly5tb3JpYXJ0eS1kZXYvcnVudGltZS9zaGVybG9jay1wYXJhbGxlbC0yMDI2MTAxNi9rLWFjdHVhbC0wMS9jb21waWxlLWlucHV0L21vZGVscy9zaGVybG9jay9rL21haW4uazoxMDI6OC0xMDQ6NDEKICAgIE9uIGNvbmZpZ3VyYXRpb246CiAgICAgIFxhbmR7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oCiAgICAgICAgICAvKiB0ZXJtOiAqLwogICAgICAgICAgTGJsJy1MVC0nZ2VuZXJhdGVkVG9wJy1HVC0ne30oCiAgICAgICAgICAgICAgTGJsJy1MVC0nayctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBrc2Vxe30oCiAgICAgICAgICAgICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlDb250cm9se30sIFNvcnRLSXRlbXt9fSgKICAgICAgICAgICAgICAgICAgICAgICAgICBMYmx1aUV2YWwnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJQ29udHJvbCdVbmRzJ1VJRXhwcnt9KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICBMYmx1aUxpdGVyYWwnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSUV4cHInVW5kcydVSVZhbHVle30oCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlVSW50e30sIFNvcnRVSVZhbHVle319KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIExibHVpVSdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJVUludCdVbmRzJ0ludHt9KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiNyIpCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICAgICAgICApCiAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgKSwKICAgICAgICAgICAgICAgICAgICAgIGRvdGt7fSgpCiAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2VudictR1QtJ3t9KC8qIEludGVybmFsTWFwOiAqLyBMYmwnU3RvcCdNYXB7fSgpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdzdGFjayctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aU5vRnJhbWVzJ0xQYXJSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJRnJhbWVze30oKQogICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgTGJsJy1MVC0nZGVwdGgnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdzdGFja0xpbWl0Jy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjgiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nY29kZSctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aU5vRnVucydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlGdW5ze30oKQogICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgTGJsJy1MVC0nZnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIxIikpLAogICAgICAgICAgICAgIExibCctTFQtJ2luaXRpYWxGdWVsJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjEiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nb3V0Jy1HVC0ne30oCiAgICAgICAgICAgICAgICAgIExibHVpUGVuZGluZydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSU91dHB1dHt9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2dlbmVyYXRlZENvdW50ZXInLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKQogICAgICAgICAgKSwKICAgICAgXGFuZHtTb3J0R2VuZXJhdGVkVG9wQ2VsbHt9fSgKICAgICAgICAgIC8qIHByZWRpY2F0ZTogKi8KICAgICAgICAgIFx0b3B7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oKSwKICAgICAgICAgIC8qIHN1YnN0aXR1dGlvbjogKi8KICAgICAgICAgIFx0b3B7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oKQogICAgICApKQprb3JlLWV4ZWM6IFdhcm5pbmcgKFdhcm5EZXB0aExpbWl0RXhjZWVkZWQpOgogICAgVGhlIGRlcHRoIGxpbWl0IDEgd2FzIGV4Y2VlZGVkLgprb3JlLWV4ZWM6IEluZm8gKEluZm9FeGVjRGVwdGgpOgogICAgZXhlY3V0aW9uIGNvbXBsZXRlOiBleGVjIGRlcHRoOiAxCg=='
KORE_05 = 'TGJsJy1MVC0nZ2VuZXJhdGVkVG9wJy1HVC0ne30oCiAgICBMYmwnLUxULSdrJy1HVC0ne30oCiAgICAgICAga3NlcXt9KAogICAgICAgICAgICAvKiBJbmo6ICovIGluantTb3J0VUlDb250cm9se30sIFNvcnRLSXRlbXt9fSgKICAgICAgICAgICAgICAgIExibHVpUmV0J0xQYXJVbmRzUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSUNvbnRyb2wnVW5kcydVSVZhbHVle30oCiAgICAgICAgICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJVUludHt9LCBTb3J0VUlWYWx1ZXt9fSgKICAgICAgICAgICAgICAgICAgICAgICAgTGJsdWlVJ0xQYXJVbmRzUlBhclVuZHMnTU9SSUFSVFktVUktU1lOVEFYJ1VuZHMnVUlVSW50J1VuZHMnSW50e30oCiAgICAgICAgICAgICAgICAgICAgICAgICAgICBcZHZ7U29ydEludHt9fSgiNyIpCiAgICAgICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICAgICApCiAgICAgICAgICAgICAgICApCiAgICAgICAgICAgICksCiAgICAgICAgICAgIGRvdGt7fSgpCiAgICAgICAgKQogICAgKSwKICAgIExibCctTFQtJ2VudictR1QtJ3t9KC8qIEludGVybmFsTWFwOiAqLyBMYmwnU3RvcCdNYXB7fSgpKSwKICAgIExibCctTFQtJ3N0YWNrJy1HVC0ne30oCiAgICAgICAgTGJsdWlOb0ZyYW1lcydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSUZyYW1lc3t9KCkKICAgICksCiAgICBMYmwnLUxULSdkZXB0aCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIwIikpLAogICAgTGJsJy1MVC0nc3RhY2tMaW1pdCctR1QtJ3t9KFxkdntTb3J0SW50e319KCI4IikpLAogICAgTGJsJy1MVC0nY29kZSctR1QtJ3t9KAogICAgICAgIExibHVpTm9GdW5zJ0xQYXJSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSUZ1bnN7fSgpCiAgICApLAogICAgTGJsJy1MVC0nZnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIwIikpLAogICAgTGJsJy1MVC0naW5pdGlhbEZ1ZWwnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMSIpKSwKICAgIExibCctTFQtJ291dCctR1QtJ3t9KAogICAgICAgIExibHVpUGVuZGluZydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSU91dHB1dHt9KCkKICAgICksCiAgICBMYmwnLUxULSdnZW5lcmF0ZWRDb3VudGVyJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSkKKQ=='
LOG_06 = 'a29yZS1leGVjOiBEZWJ1ZyAoRGVidWdBcHBsaWVkUmV3cml0ZVJ1bGVzKToKICAgIFRoZSBydWxlcyBhdCBmb2xsb3dpbmcgbG9jYXRpb25zIHdlcmUgYXBwbGllZDoKICAgIC9ob21lL2NoYXJsL01vcmlhcnR5Ly5tb3JpYXJ0eS1kZXYvcnVudGltZS9zaGVybG9jay1wYXJhbGxlbC0yMDI2MTAxNi9rLWFjdHVhbC0wMS9jb21waWxlLWlucHV0L21vZGVscy9zaGVybG9jay9rL21haW4uazo4MDo4LTgxOjQ1CiAgICBPbiBjb25maWd1cmF0aW9uOgogICAgICBcYW5ke1NvcnRHZW5lcmF0ZWRUb3BDZWxse319KAogICAgICAgICAgLyogdGVybTogKi8KICAgICAgICAgIExibCctTFQtJ2dlbmVyYXRlZFRvcCctR1QtJ3t9KAogICAgICAgICAgICAgIExibCctTFQtJ2snLUdULSd7fSgKICAgICAgICAgICAgICAgICAga3NlcXt9KAogICAgICAgICAgICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJQ29udHJvbHt9LCBTb3J0S0l0ZW17fX0oCiAgICAgICAgICAgICAgICAgICAgICAgICAgTGJsdWlSZXQnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJQ29udHJvbCdVbmRzJ1VJVmFsdWV7fSgKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJVUludHt9LCBTb3J0VUlWYWx1ZXt9fSgKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIExibHVpVSdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJVUludCdVbmRzJ0ludHt9KAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIFxkdntTb3J0SW50e319KCI3IikKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgICAgICAgICBkb3Rre30oKQogICAgICAgICAgICAgICAgICApCiAgICAgICAgICAgICAgKSwKICAgICAgICAgICAgICBMYmwnLUxULSdlbnYnLUdULSd7fSgvKiBJbnRlcm5hbE1hcDogKi8gTGJsJ1N0b3AnTWFwe30oKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nc3RhY2snLUdULSd7fSgKICAgICAgICAgICAgICAgICAgTGJsdWlOb0ZyYW1lcydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSUZyYW1lc3t9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2RlcHRoJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nc3RhY2tMaW1pdCctR1QtJ3t9KFxkdntTb3J0SW50e319KCI4IikpLAogICAgICAgICAgICAgIExibCctTFQtJ2NvZGUnLUdULSd7fSgKICAgICAgICAgICAgICAgICAgTGJsdWlOb0Z1bnMnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJRnVuc3t9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2Z1ZWwnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdpbml0aWFsRnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIxIikpLAogICAgICAgICAgICAgIExibCctTFQtJ291dCctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aVBlbmRpbmcnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJJ1VuZHMnVUlPdXRwdXR7fSgpCiAgICAgICAgICAgICAgKSwKICAgICAgICAgICAgICBMYmwnLUxULSdnZW5lcmF0ZWRDb3VudGVyJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSkKICAgICAgICAgICksCiAgICAgIFxhbmR7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oCiAgICAgICAgICAvKiBwcmVkaWNhdGU6ICovCiAgICAgICAgICBcdG9we1NvcnRHZW5lcmF0ZWRUb3BDZWxse319KCksCiAgICAgICAgICAvKiBzdWJzdGl0dXRpb246ICovCiAgICAgICAgICBcdG9we1NvcnRHZW5lcmF0ZWRUb3BDZWxse319KCkKICAgICAgKSkKa29yZS1leGVjOiBXYXJuaW5nIChXYXJuRGVwdGhMaW1pdEV4Y2VlZGVkKToKICAgIFRoZSBkZXB0aCBsaW1pdCAxIHdhcyBleGNlZWRlZC4Ka29yZS1leGVjOiBJbmZvIChJbmZvRXhlY0RlcHRoKToKICAgIGV4ZWN1dGlvbiBjb21wbGV0ZTogZXhlYyBkZXB0aDogMQo='
# The transport records identical base64 strings for 06/07 state.kore and 11/12 stdout.
KORE_06 = 'TGJsJy1MVC0nZ2VuZXJhdGVkVG9wJy1HVC0ne30oCiAgICBMYmwnLUxULSdrJy1HVC0ne30oZG90a3t9KCkpLAogICAgTGJsJy1MVC0nZW52Jy1HVC0ne30oLyogSW50ZXJuYWxNYXA6ICovIExibCdTdG9wJ01hcHt9KCkpLAogICAgTGJsJy1MVC0nc3RhY2snLUdULSd7fSgKICAgICAgICBMYmx1aU5vRnJhbWVzJ0xQYXJSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJRnJhbWVze30oKQogICAgKSwKICAgIExibCctTFQtJ2RlcHRoJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICBMYmwnLUxULSdzdGFja0xpbWl0Jy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjgiKSksCiAgICBMYmwnLUxULSdjb2RlJy1HVC0ne30oCiAgICAgICAgTGJsdWlOb0Z1bnMnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJRnVuc3t9KCkKICAgICksCiAgICBMYmwnLUxULSdmdWVsJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICBMYmwnLUxULSdpbml0aWFsRnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIxIikpLAogICAgTGJsJy1MVC0nb3V0Jy1HVC0ne30oCiAgICAgICAgTGJsdWlEb25lJ0xQYXJVbmRzUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSU91dHB1dCdVbmRzJ1VJVmFsdWV7fSgKICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJVUludHt9LCBTb3J0VUlWYWx1ZXt9fSgKICAgICAgICAgICAgICAgIExibHVpVSdMUGFyVW5kc1JQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJVUludCdVbmRzJ0ludHt9KAogICAgICAgICAgICAgICAgICAgIFxkdntTb3J0SW50e319KCI3IikKICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgKQogICAgICAgICkKICAgICksCiAgICBMYmwnLUxULSdnZW5lcmF0ZWRDb3VudGVyJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSkKKQ=='
LOG_07 = 'a29yZS1leGVjOiBEZWJ1ZyAoRGVidWdBcHBsaWVkUmV3cml0ZVJ1bGVzKToKICAgIE5vIHJ1bGVzIHdlcmUgYXBwbGllZC4KICAgIE9uIGNvbmZpZ3VyYXRpb246CiAgICAgIFxhbmR7U29ydEdlbmVyYXRlZFRvcENlbGx7fX0oCiAgICAgICAgICAvKiB0ZXJtOiAqLwogICAgICAgICAgTGJsJy1MVC0nZ2VuZXJhdGVkVG9wJy1HVC0ne30oCiAgICAgICAgICAgICAgTGJsJy1MVC0nayctR1QtJ3t9KGRvdGt7fSgpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdlbnYnLUdULSd7fSgvKiBJbnRlcm5hbE1hcDogKi8gTGJsJ1N0b3AnTWFwe30oKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nc3RhY2snLUdULSd7fSgKICAgICAgICAgICAgICAgICAgTGJsdWlOb0ZyYW1lcydMUGFyUlBhclVuZHMnTU9SSUFSVFktVUknVW5kcydVSUZyYW1lc3t9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2RlcHRoJy1HVC0ne30oXGR2e1NvcnRJbnR7fX0oIjAiKSksCiAgICAgICAgICAgICAgTGJsJy1MVC0nc3RhY2tMaW1pdCctR1QtJ3t9KFxkdntTb3J0SW50e319KCI4IikpLAogICAgICAgICAgICAgIExibCctTFQtJ2NvZGUnLUdULSd7fSgKICAgICAgICAgICAgICAgICAgTGJsdWlOb0Z1bnMnTFBhclJQYXJVbmRzJ01PUklBUlRZLVVJLVNZTlRBWCdVbmRzJ1VJRnVuc3t9KCkKICAgICAgICAgICAgICApLAogICAgICAgICAgICAgIExibCctTFQtJ2Z1ZWwnLUdULSd7fShcZHZ7U29ydEludHt9fSgiMCIpKSwKICAgICAgICAgICAgICBMYmwnLUxULSdpbml0aWFsRnVlbCctR1QtJ3t9KFxkdntTb3J0SW50e319KCIxIikpLAogICAgICAgICAgICAgIExibCctTFQtJ291dCctR1QtJ3t9KAogICAgICAgICAgICAgICAgICBMYmx1aURvbmUnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSSdVbmRzJ1VJT3V0cHV0J1VuZHMnVUlWYWx1ZXt9KAogICAgICAgICAgICAgICAgICAgICAgLyogSW5qOiAqLyBpbmp7U29ydFVJVUludHt9LCBTb3J0VUlWYWx1ZXt9fSgKICAgICAgICAgICAgICAgICAgICAgICAgICBMYmx1aVUnTFBhclVuZHNSUGFyVW5kcydNT1JJQVJUWS1VSS1TWU5UQVgnVW5kcydVSVVJbnQnVW5kcydJbnR7fSgKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgXGR2e1NvcnRJbnR7fX0oIjciKQogICAgICAgICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICAgICAgICkKICAgICAgICAgICAgICAgICAgKQogICAgICAgICAgICAgICksCiAgICAgICAgICAgICAgTGJsJy1MVC0nZ2VuZXJhdGVkQ291bnRlcictR1QtJ3t9KFxkdntTb3J0SW50e319KCIwIikpCiAgICAgICAgICApLAogICAgICBcYW5ke1NvcnRHZW5lcmF0ZWRUb3BDZWxse319KAogICAgICAgICAgLyogcHJlZGljYXRlOiAqLwogICAgICAgICAgXHRvcHtTb3J0R2VuZXJhdGVkVG9wQ2VsbHt9fSgpLAogICAgICAgICAgLyogc3Vic3RpdHV0aW9uOiAqLwogICAgICAgICAgXHRvcHtTb3J0R2VuZXJhdGVkVG9wQ2VsbHt9fSgpCiAgICAgICkpCmtvcmUtZXhlYzogSW5mbyAoSW5mb0V4ZWNEZXB0aCk6CiAgICBleGVjdXRpb24gY29tcGxldGU6IGV4ZWMgZGVwdGg6IDAK'
PRETTY_08 = 'aW5pdEdlbmVyYXRlZFRvcENlbGwgKCAkUEdNIHwtPiB1aVJ1biAoIHVpTm9GdW5zICggKSAsIHVpTGl0ZXJhbCAoIHVpVSAoIDcgKSApICwgMSAsIDggKSApCg=='
PRETTY_09 = 'PGdlbmVyYXRlZFRvcD4KICA8az4KICAgIHVpRXZhbCAoIHVpTGl0ZXJhbCAoIHVpVSAoIDcgKSApICkgfj4gLksKICA8L2s+CiAgPGVudj4KICAgIC5NYXAKICA8L2Vudj4KICA8c3RhY2s+CiAgICB1aU5vRnJhbWVzICggKQogIDwvc3RhY2s+CiAgPGRlcHRoPgogICAgMAogIDwvZGVwdGg+CiAgPHN0YWNrTGltaXQ+CiAgICA4CiAgPC9zdGFja0xpbWl0PgogIDxjb2RlPgogICAgdWlOb0Z1bnMgKCApCiAgPC9jb2RlPgogIDxmdWVsPgogICAgMQogIDwvZnVlbD4KICA8aW5pdGlhbEZ1ZWw+CiAgICAxCiAgPC9pbml0aWFsRnVlbD4KICA8b3V0PgogICAgdWlQZW5kaW5nICggKQogIDwvb3V0Pgo8L2dlbmVyYXRlZFRvcD4K'
PRETTY_10 = 'PGdlbmVyYXRlZFRvcD4KICA8az4KICAgIHVpUmV0ICggdWlVICggNyApICkgfj4gLksKICA8L2s+CiAgPGVudj4KICAgIC5NYXAKICA8L2Vudj4KICA8c3RhY2s+CiAgICB1aU5vRnJhbWVzICggKQogIDwvc3RhY2s+CiAgPGRlcHRoPgogICAgMAogIDwvZGVwdGg+CiAgPHN0YWNrTGltaXQ+CiAgICA4CiAgPC9zdGFja0xpbWl0PgogIDxjb2RlPgogICAgdWlOb0Z1bnMgKCApCiAgPC9jb2RlPgogIDxmdWVsPgogICAgMAogIDwvZnVlbD4KICA8aW5pdGlhbEZ1ZWw+CiAgICAxCiAgPC9pbml0aWFsRnVlbD4KICA8b3V0PgogICAgdWlQZW5kaW5nICggKQogIDwvb3V0Pgo8L2dlbmVyYXRlZFRvcD4K'
PRETTY_11 = 'PGdlbmVyYXRlZFRvcD4KICA8az4KICAgIC5LCiAgPC9rPgogIDxlbnY+CiAgICAuTWFwCiAgPC9lbnY+CiAgPHN0YWNrPgogICAgdWlOb0ZyYW1lcyAoICkKICA8L3N0YWNrPgogIDxkZXB0aD4KICAgIDAKICA8L2RlcHRoPgogIDxzdGFja0xpbWl0PgogICAgOAogIDwvc3RhY2tMaW1pdD4KICA8Y29kZT4KICAgIHVpTm9GdW5zICggKQogIDwvY29kZT4KICA8ZnVlbD4KICAgIDAKICA8L2Z1ZWw+CiAgPGluaXRpYWxGdWVsPgogICAgMQogIDwvaW5pdGlhbEZ1ZWw+CiAgPG91dD4KICAgIHVpRG9uZSAoIHVpVSAoIDcgKSApCiAgPC9vdXQ+CjwvZ2VuZXJhdGVkVG9wPgo='

FIXTURES = {
    '03-exec-depth0/stderr': LOG_03, '03-exec-depth0/state.kore': KORE_03,
    '04-exec-depth1-1/stderr': LOG_04, '04-exec-depth1-1/state.kore': KORE_04,
    '05-exec-depth1-2/stderr': LOG_05, '05-exec-depth1-2/state.kore': KORE_05,
    '06-exec-depth1-3/stderr': LOG_06, '06-exec-depth1-3/state.kore': KORE_06,
    '07-exec-depth1-4/stderr': LOG_07, '07-exec-depth1-4/state.kore': KORE_06,
    '08-kprint-03/stdout': PRETTY_08, '09-kprint-04/stdout': PRETTY_09,
    '10-kprint-05/stdout': PRETTY_10, '11-kprint-06/stdout': PRETTY_11,
    '12-kprint-07/stdout': PRETTY_11,
}
RECORDED = {
    '03-exec-depth0/stderr': (
        152, '8a23f256a5bb1b5195afd568ce2ad5b399b27129f3d356ddbe3597f54b39c345'),
    '03-exec-depth0/state.kore': (
        942, '3cc859801eb50e7ccb3cd06f67c5f563d77d091eeabd5ca80b12b18e50e00d5b'),
    '04-exec-depth1-1/stderr': (
        2530, '820fd477d15bbac4c6910d710719ae06ad9b930b94705ac6ea23eca21d8c6668'),
    '04-exec-depth1-1/state.kore': (
        1304, '9a4aaf59282b15de6d812b4458c73ce3a165b42a3d9ae2c0fcac2211c36a7315'),
    '05-exec-depth1-2/stderr': (
        2287, '4b2b0b65a4e4d2f83e82dcfc59e3be755305a09631df97d2d7515b7f9cb15631'),
    '05-exec-depth1-2/state.kore': (
        1165, '364dee4b3b7cf0bfbb7b67b3fb320d2f93035010026552a7e44767b38c8b0a72'),
    '06-exec-depth1-3/stderr': (
        2126, '791e1f0730eb990abb7c8a6ca75b414a3ee302fb8dfa93a6bb6004786ad616f4'),
    '06-exec-depth1-3/state.kore': (
        928, '9dfc031505d556ce190d906a4d54fb945ee27c88d1aeef407edef87e09cb2e53'),
    '07-exec-depth1-4/stderr': (
        1578, 'e6f4fcb277679787d64198b1955fd03930bd2a8b8b6f2777ee5cfdeae2d06606'),
    '07-exec-depth1-4/state.kore': (
        928, '9dfc031505d556ce190d906a4d54fb945ee27c88d1aeef407edef87e09cb2e53'),
    '08-kprint-03/stdout': (
        91, '01999cdda4fa81863ed9d28bb0dbbff692d507644b93174410cb756120b1e1fc'),
    '09-kprint-04/stdout': (
        354, '7bf8c8cd069f87d729bd6b2eeafab6228b97c82862d95d5df9c9e178162468be'),
    '10-kprint-05/stdout': (
        339, 'ae1efa83b1ea93950bd643261dca7af44b212ab229f51ab0484ce2efce7b351d'),
    '11-kprint-06/stdout': (
        323, 'cf32b5742930499fbd3ab751ef5761cc443cfa11f5be21b12d61a812ded66a01'),
    '12-kprint-07/stdout': (
        323, 'cf32b5742930499fbd3ab751ef5761cc443cfa11f5be21b12d61a812ded66a01'),
}
INVENTORY = (
    (74, 8, 77, 41, 'cb548a8fdb5141fb39e3d6e31c6d1aa092757e27088afd822fbb69ff1f026503'),
    (78, 8, 79, 38, 'd2efeabc4b964a09776d430f72d9d0b12aa120d6d16644ed671a3ce859850e84'),
    (80, 8, 81, 45, '8a511f639830645a8b25dccfe5629aacac457a9b631e6f4f604a60120b9895f8'),
    (82, 8, 83, 49, '084079a7732e682c2d05a3427cdf7ac5ffae4fa0ecd372d417f54e2ced0e870c'),
    (84, 8, 86, 38, '3c32fdef90b58f35b7628a92ecf04ff22fc8fa8d658866f6a1046504ce7f4ca8'),
    (102, 8, 104, 41, '24e4f7636821d9b6c2ff9c556df6d0ead3fecb3d61a12115b48f0a40806d40e2'),
    (105, 8, 107, 49, 'c3b3964c384386351651be0cba2ac8eda6157cf8920baa6dd1641d2db1fc3575'),
)

# (kore-exec stage, its --pattern stage, kprint stage of its output), as recorded.
STARTUP = ('04-exec-depth1-1', '03-exec-depth0', '09-kprint-04')
EDGES = (('05-exec-depth1-2', '04-exec-depth1-1', '10-kprint-05'),
         ('06-exec-depth1-3', '05-exec-depth1-2', '11-kprint-06'))
TERMINAL = ('07-exec-depth1-4', '06-exec-depth1-3', '12-kprint-07')

WARN_ENTRY = b'kore-exec: Warning (WarnDepthLimitExceeded):\n    The depth limit 1 was exceeded.\n'
DEPTH_1 = b'kore-exec: Info (InfoExecDepth):\n    execution complete: exec depth: 1\n'
DEPTH_0 = b'kore-exec: Info (InfoExecDepth):\n    execution complete: exec depth: 0\n'
APPLIED = b'    The rules at following locations were applied:\n'
NO_RULES = b'    No rules were applied.\n'
OTHER = hashlib.sha256(b'other').hexdigest()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def actual(name):
    return base64.b64decode(FIXTURES[name], validate=True)


def pretty(stage):
    return actual(stage + '/stdout').decode('ascii')


def replaced(data, old, new):
    """Synthetic mutation of a copy of an actual fixture; the anchor must occur exactly once."""
    if data.count(old) != 1:
        raise AssertionError(f'mutation anchor occurs {data.count(old)} times: {old!r}')
    return data.replace(old, new)


def location(span):
    return b'    ' + SOURCE.encode('ascii') + b':' + span + b'\n'


def state_path(stage):
    return f'{STAGES}/{stage}/state.kore'


def inventory(source=SOURCE):
    return [dict(source_path=source, start_line=first, start_column=column, end_line=last,
                 end_column=final, unique_id=unique)
            for first, column, last, final, unique in INVENTORY]


def engine_receipt(stage, pattern):
    """Recorded kore-exec command, return code and artifact hashes of one receipt stage."""
    return dict(argv=[KORE_EXEC, KOMPILED + '/definition.kore', '--module', 'MORIARTY-UI',
                      '--pattern', state_path(pattern), '--output', state_path(stage),
                      '--depth', '1', '--smt', 'none', '--log-format', 'standard',
                      '--disable-log-timestamps', '--log-entries',
                      'DebugAppliedRewriteRules,InfoExecDepth'],
                exit_status=0, stdout_sha256=EMPTY, stderr_sha256=RECORDED[stage + '/stderr'][1],
                output_sha256=RECORDED[stage + '/state.kore'][1])


def print_receipt(printer, stage):
    """Recorded kprint command, return code and artifact hashes of one receipt stage."""
    return dict(argv=[KPRINT, KOMPILED, state_path(stage), 'false', 'false'], exit_status=0,
                stdout_sha256=RECORDED[printer + '/stdout'][1], stderr_sha256=EMPTY)


def probe(spec):
    stage, pattern, printer = spec
    return dict(configuration=pretty(printer), log=actual(stage + '/stderr'),
                state_kore=actual(stage + '/state.kore'),
                engine_receipt=engine_receipt(stage, pattern),
                print_receipt=print_receipt(printer, stage))


def captures():
    rows = [dict(configuration=pretty('09-kprint-04'))]
    for spec in EDGES:
        row = probe(spec)
        row['incoming_log'] = row.pop('log')
        rows.append(row)
    return rows


def run(rows=None, **overrides):
    options = dict(source_path=SOURCE, rule_spans=inventory(), terminal_probe=probe(TERMINAL),
                   startup_probe=probe(STARTUP), run_input=RUN_INPUT)
    options.update(overrides)
    return k.decode_capture('N01', ENTRY, captures() if rows is None else rows, **options)


def relogged(step, raw, key='log'):
    """Replace a step's log and re-declare its stderr hash, so only the targeted guard fires."""
    step[key] = raw
    step['engine_receipt']['stderr_sha256'] = sha(raw)
    return step


def reprinted(step, text):
    step['configuration'] = text
    step['print_receipt']['stdout_sha256'] = sha(text.encode('ascii'))
    return step


def set_option(receipt, flag, value):
    argv = receipt['argv']
    argv[argv.index(flag) + 1] = value


def logged_body(state_kore):
    """Observed On-configuration layout: input Kore re-indented inside the \\and/\\top wrapper."""
    term = ['          ' + line for line in state_kore.decode('ascii').split('\n')]
    term[-1] += ','
    return '\n'.join(['      \\and{SortGeneratedTopCell{}}(', '          /* term: */', *term,
                      '      \\and{SortGeneratedTopCell{}}(', '          /* predicate: */',
                      '          \\top{SortGeneratedTopCell{}}(),',
                      '          /* substitution: */', '          \\top{SortGeneratedTopCell{}}()',
                      '      ))']) + '\n'


def prefixed(result, prefix):
    return [gap for gap in result['gaps'] if gap.startswith(prefix)]


class ActualN01CaptureTests(unittest.TestCase):
    def setUp(self):
        self.assertIsNotNone(k, 'fixed capture adapter module is absent')

    def assert_edge_rejected(self, raw, *gaps):
        rows = captures()
        relogged(rows[1], raw, 'incoming_log')
        result = run(rows)
        self.assertFalse(result['comparison_ready'])
        for gap in gaps:
            self.assertIn('edge-0-' + gap, result['gaps'])
        self.assertEqual([row['rule_start_line'] for row in
                          result['observation']['records']], [None, None])
        self.assertIsNone(result['observed_rule_identities'][0])
        return result

    def assert_terminal_rejected(self, terminal, *gaps):
        result = run(terminal_probe=terminal)
        self.assertFalse(result['comparison_ready'])
        for gap in gaps:
            self.assertIn('terminal-' + gap, result['gaps'])
        self.assertIsNot(result['observation']['terminal_has_next'], False)
        return result

    def assert_startup_rejected(self, startup, *gaps):
        result = run(startup_probe=startup)
        self.assertFalse(result['comparison_ready'])
        for gap in gaps:
            self.assertIn('startup-' + gap, result['gaps'])
        self.assertIsNone(result['startup_probe']['rule_start_line'])
        self.assertIsNone(result['startup_probe']['rule'])
        return result

    # Actual fixtures: provenance, fixed sources and the observed N01 layout.

    def test_actual_fixture_provenance(self):
        self.assertEqual(set(FIXTURES), set(RECORDED))
        for name, (size, digest) in RECORDED.items():
            with self.subTest(fixture=name):
                data = actual(name)
                self.assertEqual(len(data), size)
                self.assertEqual(sha(data), digest)
        self.assertEqual(actual('07-exec-depth1-4/state.kore'),
                         actual('06-exec-depth1-3/state.kore'))
        self.assertEqual(actual('12-kprint-07/stdout'), actual('11-kprint-06/stdout'))
        raw = RUN_INPUT.encode('ascii')
        self.assertEqual((len(raw), sha(raw)), RUN_INPUT_RECORD)
        self.assertEqual(sha(b''), EMPTY)
        self.assertEqual(k.EMPTY_SHA256, EMPTY)

    def test_fixed_adapter_and_shared_sources(self):
        self.assertEqual(set(k.ADAPTER_SOURCES), set(FIXED_SOURCES))
        for name, (size, digest) in FIXED_SOURCES.items():
            with self.subTest(source=name):
                data = MODULE.with_name(name).read_bytes()
                self.assertEqual((len(data), sha(data)), (size, digest))
                self.assertEqual(k.ADAPTER_SOURCES[name], digest)
        self.assertEqual(k.FORMAT, 'sherlock-k-standard-log-capture/2')
        self.assertEqual(k.LOG_FORMAT, 'installed-kore-standard-log/2')
        self.assertIn('not bound', run()['evidence']['adapter_source_binding'])

    def test_actual_logs_decode_to_the_observed_layout(self):
        for stage, span in (('04-exec-depth1-1', (74, 8, 77, 41)),
                            ('05-exec-depth1-2', (102, 8, 104, 41)),
                            ('06-exec-depth1-3', (80, 8, 81, 45))):
            with self.subTest(stage=stage):
                raw = actual(stage + '/stderr')
                self.assertIn(location('{}:{}-{}:{}'.format(*span).encode('ascii')), raw)
                decoded = k.decode_log(raw, source_path=SOURCE)
                self.assertEqual(decoded['gaps'], [])
                self.assertEqual([(entry['severity'], entry['kind'])
                                  for entry in decoded['raw_entries']],
                                 [('Debug', 'DebugAppliedRewriteRules'),
                                  ('Warning', 'WarnDepthLimitExceeded'),
                                  ('Info', 'InfoExecDepth')])
                self.assertEqual(decoded['depth_limit_warnings'], [1])
                self.assertEqual(decoded['execution_depths'], [1])
                self.assertEqual(decoded['applied_entries'][0]['variant'], 'applied')
                self.assertEqual(decoded['applied_rule_locations'], [dict(
                    source=SOURCE, start_line=span[0], start_column=span[1], end_line=span[2],
                    end_column=span[3])])
                self.assertEqual(base64.b64decode(decoded['raw_log']['base64']), raw)
                self.assertEqual((decoded['raw_log']['size'], decoded['raw_sha256']),
                                 RECORDED[stage + '/stderr'])
        terminal = k.decode_log(actual('07-exec-depth1-4/stderr'), source_path=SOURCE)
        self.assertEqual(terminal['gaps'], [])
        self.assertEqual([entry['kind'] for entry in terminal['raw_entries']],
                         ['DebugAppliedRewriteRules', 'InfoExecDepth'])
        self.assertEqual(terminal['applied_entries'][0]['variant'], 'no-rules')
        self.assertEqual(terminal['applied_rule_locations'], [])
        self.assertEqual(terminal['depth_limit_warnings'], [])
        self.assertEqual(terminal['execution_depths'], [0])

    def test_actual_logged_terms_are_previous_output_kore_except_at_startup(self):
        for stage, previous in (('05-exec-depth1-2', '04-exec-depth1-1'),
                                ('06-exec-depth1-3', '05-exec-depth1-2'),
                                ('07-exec-depth1-4', '06-exec-depth1-3')):
            with self.subTest(stage=stage):
                entry = k.decode_log(actual(stage + '/stderr'),
                                     source_path=SOURCE)['applied_entries'][0]
                self.assertEqual(entry['configuration_text'],
                                 logged_body(actual(previous + '/state.kore')))
        startup = k.decode_log(actual('04-exec-depth1-1/stderr'),
                               source_path=SOURCE)['applied_entries'][0]
        self.assertNotEqual(startup['configuration_text'],
                            logged_body(actual('03-exec-depth0/state.kore')))

    def test_actual_n01_capture_is_comparison_ready_without_authentication(self):
        result = run()
        self.assertEqual(result['gaps'], [])
        self.assertTrue(result['comparison_ready'])
        self.assertEqual(result['format'], k.FORMAT)
        self.assertEqual(result['source_path'], SOURCE)
        observation = result['observation']
        self.assertEqual(observation['case_id'], 'N01')
        self.assertEqual([row['rule_start_line'] for row in observation['records']], [102, 80])
        self.assertEqual([row['charge'] for row in observation['records']], [1, 0])
        self.assertIs(observation['terminal_has_next'], False)
        self.assertEqual(observation['initial']['control_k'], 'uiEval(uiLiteral(uiU(7)))')
        self.assertEqual(observation['terminal']['control_k'], '.K')
        self.assertEqual(observation['terminal']['output_k'], 'uiDone(uiU(7))')
        self.assertEqual(result['observed_rule_identities'], [
            dict(source=SOURCE, start_line=102, start_column=8, end_line=104, end_column=41,
                 unique_id=INVENTORY[5][4]),
            dict(source=SOURCE, start_line=80, start_column=8, end_line=81, end_column=45,
                 unique_id=INVENTORY[2][4])])
        self.assertEqual(result['startup_probe']['rule_start_line'], 74)
        self.assertEqual(result['startup_probe']['rule'], dict(
            source=SOURCE, start_line=74, start_column=8, end_line=77, end_column=41,
            unique_id=INVENTORY[0][4]))
        self.assertEqual(result['run_input']['raw_sha256'], RUN_INPUT_RECORD[1])
        for index, row in enumerate(captures()[1:], 1):
            raw = result['raw_captures'][index]
            self.assertEqual(base64.b64decode(raw['incoming_log']['base64']), row['incoming_log'])
            self.assertEqual(base64.b64decode(raw['state_kore']['base64']), row['state_kore'])
            self.assertEqual(raw['engine_receipt'], row['engine_receipt'])
            self.assertEqual(raw['print_receipt'], row['print_receipt'])
        self.assertEqual(result['raw_captures'][0], dict(configuration=pretty('09-kprint-04')))
        self.assertEqual(base64.b64decode(result['raw_startup_probe']['log']['base64']),
                         actual('04-exec-depth1-1/stderr'))
        self.assertIn('not-established', result['execution_authenticity'])
        self.assertEqual(result['evidence']['producer_authenticity'], 'not-established')
        self.assertIn('not authenticated', result['evidence']['receipts'])
        self.assertIn('blocked', result['evidence']['reliance'])
        self.assertEqual(result['clock']['startup'], 'separate-observed-startup-probe-required')

    def test_actual_edge_links_keep_engine_and_print_hashes_distinct(self):
        links = run()['edge_links']
        for link, (stage, pattern, printer) in zip(links, EDGES):
            with self.subTest(stage=stage):
                self.assertEqual(link['input_kore_sha256'], RECORDED[pattern + '/state.kore'][1])
                self.assertEqual(link['engine_pattern_path'], state_path(pattern))
                self.assertEqual(link['engine_output_path'], state_path(stage))
                self.assertEqual(link['print_input_path'], state_path(stage))
                self.assertEqual(link['engine_log_sha256'], RECORDED[stage + '/stderr'][1])
                self.assertEqual(link['engine_output_kore_sha256'],
                                 RECORDED[stage + '/state.kore'][1])
                self.assertEqual(link['print_stdout_configuration_sha256'],
                                 RECORDED[printer + '/stdout'][1])
                self.assertNotEqual(link['engine_output_kore_sha256'],
                                    link['print_stdout_configuration_sha256'])

    def test_startup_before_is_retained_logged_kore_never_typed(self):
        startup = run()['startup_probe']
        self.assertEqual(startup['gaps'], [])
        self.assertIsNone(startup['before'])
        self.assertTrue(startup['before_projection'].startswith('not-established'))
        lines = actual('04-exec-depth1-1/stderr').decode('ascii').split('\n')
        body = '\n'.join(lines[lines.index('    On configuration:') + 1:
                               lines.index('kore-exec: Warning (WarnDepthLimitExceeded):')]) + '\n'
        before = startup['before_kore']
        self.assertEqual(before['text'], body)
        self.assertEqual(before['sha256'], sha(body.encode('ascii')))
        self.assertEqual(before['log_first_line'], 1)
        self.assertIn("LbluiRun'", before['text'])
        self.assertIn("Lbl'-LT-'generatedCounter'-GT-'", before['text'])
        self.assertEqual(startup['after']['original_configuration'], pretty('09-kprint-04'))
        self.assertIn('not compared with input bytes', startup['limitation'])

    def test_depth0_initializer_output_is_not_typed_or_normalized(self):
        depth0_kore = actual('03-exec-depth0/state.kore')
        self.assertTrue(depth0_kore.startswith(b'LblinitGeneratedTopCell{}('))
        self.assertNotIn(b"Lbl'-LT-'generatedTop'-GT-'", depth0_kore)
        depth0_log = k.decode_log(actual('03-exec-depth0/stderr'), source_path=SOURCE)
        self.assertEqual(depth0_log['gaps'], [])
        self.assertEqual(depth0_log['applied_entries'], [])
        self.assertEqual(depth0_log['depth_limit_warnings'], [0])
        self.assertEqual(depth0_log['execution_depths'], [0])
        depth0_pretty = pretty('08-kprint-03')
        self.assertTrue(depth0_pretty.startswith('initGeneratedTopCell ( $PGM |-> uiRun ('))
        with self.assertRaises(ValueError):
            k.decode_capture('N01', ENTRY, [dict(configuration=depth0_pretty)],
                             source_path=SOURCE)
        with self.assertRaises(ValueError):
            k.decode_startup_probe(probe(STARTUP), post_startup_configuration=depth0_pretty,
                                   entry_k=ENTRY, source_path=SOURCE, rule_spans=inventory())
        before = run()['startup_probe']['before_kore']['text']
        self.assertNotIn('LblinitGeneratedTopCell', before)
        self.assertNotEqual(before, logged_body(depth0_kore))

    def test_startup_probe_api(self):
        result = k.decode_startup_probe(probe(STARTUP),
                                        post_startup_configuration=pretty('09-kprint-04'),
                                        entry_k=ENTRY, source_path=SOURCE,
                                        rule_spans=inventory())
        self.assertEqual(result['gaps'], [])
        self.assertEqual(result['rule_start_line'], 74)
        result = k.decode_startup_probe(probe(STARTUP),
                                        post_startup_configuration=pretty('09-kprint-04'),
                                        entry_k=ENTRY, source_path=SOURCE)
        self.assertIn('startup-rule-span-inventory-not-supplied', result['gaps'])
        self.assertIsNone(result['rule'])
        result = k.decode_startup_probe(probe(STARTUP),
                                        post_startup_configuration=pretty('10-kprint-05'),
                                        entry_k=ENTRY, source_path=SOURCE,
                                        rule_spans=inventory())
        self.assertIn('startup-result-raw-differs-from-first-capture', result['gaps'])
        self.assertIn('startup-result-does-not-match-first-complete-capture', result['gaps'])
        self.assertIsNone(result['rule_start_line'])

    def test_raw_kore_carries_ten_cells_and_kprint_nine(self):
        labels = re.compile(rb"Lbl'-LT-'([A-Za-z]+)'-GT-'")
        for stage in ('04-exec-depth1-1', '05-exec-depth1-2', '06-exec-depth1-3',
                      '07-exec-depth1-4'):
            with self.subTest(stage=stage):
                names = sorted(name.decode('ascii') for name in
                               labels.findall(actual(stage + '/state.kore')))
                self.assertEqual(names, sorted(['generatedTop', 'generatedCounter', *CELLS]))
        for printer in ('09-kprint-04', '10-kprint-05', '11-kprint-06', '12-kprint-07'):
            with self.subTest(printer=printer):
                tags = re.findall(r'<([A-Za-z][A-Za-z0-9]*)>', pretty(printer))
                self.assertEqual(sorted(tags), sorted(['generatedTop', *CELLS]))
        result = run()
        self.assertEqual(result['projection_boundary']['cells'], list(CELLS))
        self.assertEqual(result['projection_boundary']['printer_omitted'], ['generatedCounter'])
        self.assertTrue(all(link['raw_kore_has_generated_counter_label']
                            for link in result['edge_links']))

    def test_only_k_unit_tail_is_reduced_and_originals_are_retained(self):
        result = run()
        tails = result['configuration_unit_tails']
        self.assertIsNone(tails[2])
        for index, printer in ((0, '09-kprint-04'), (1, '10-kprint-05')):
            with self.subTest(printer=printer):
                original = pretty(printer)
                note = tails[index]
                self.assertTrue(note['original_k'].strip().endswith(' ~> .K'))
                self.assertEqual(note['decoded_k'].strip(),
                                 note['original_k'].strip()[:-len(' ~> .K')])
                self.assertEqual(result['original_configurations'][index], original)
                self.assertEqual(result['raw_configurations'][index],
                                 original.replace(' ~> .K', '', 1))
        self.assertEqual(result['raw_configurations'][2], pretty('11-kprint-06'))
        self.assertEqual(result['startup_probe']['after']['unit_tail'], tails[0])

    def test_actual_terminal_replay_is_raw_stutter(self):
        result = run()
        terminal = result['terminal_probe']
        self.assertEqual(terminal['gaps'], [])
        self.assertIs(terminal['terminal_has_next'], False)
        self.assertEqual(terminal['decoded_log']['applied_entries'][0]['variant'], 'no-rules')
        self.assertEqual(terminal['before']['snapshot'], terminal['after']['snapshot'])
        raw = result['raw_terminal_probe']
        self.assertEqual(raw['state_kore']['sha256'],
                         result['edge_links'][-1]['engine_output_kore_sha256'])
        self.assertEqual(raw['configuration'], result['original_configurations'][-1])
        self.assertEqual(base64.b64decode(raw['log']['base64']), actual('07-exec-depth1-4/stderr'))
        direct = k.decode_terminal_probe(probe(TERMINAL),
                                         terminal_configuration=pretty('11-kprint-06'),
                                         terminal_state_kore=actual('06-exec-depth1-3/state.kore'),
                                         entry_k=ENTRY, source_path=SOURCE)
        self.assertEqual(direct['gaps'], [])
        self.assertIs(direct['terminal_has_next'], False)

    def test_actual_bundle_keeps_frozen_envelope_and_is_json(self):
        decoded = run()
        bundle = k.observation_bundle([decoded], source_sha256=MAIN_K_SHA256)
        self.assertEqual(bundle['format'], 'sherlock-k-normal-observations/1')
        self.assertEqual(set(bundle['observations']), {
            'producer', 'profile', 'source_sha256', 'boundary', 'programs', 'runs'})
        self.assertEqual(bundle['observations']['producer'], 'sherlock-k-reference')
        self.assertEqual(bundle['observations']['profile'], 'sherlock-k-a/1')
        self.assertEqual(bundle['observations']['source_sha256'], MAIN_K_SHA256)
        self.assertEqual(bundle['observations']['boundary'], 'post-startup')
        self.assertEqual(bundle['observations']['programs'], [dict(
            case_id='N01', code_k='uiNoFuns()', entry_k=ENTRY)])
        self.assertEqual(bundle['observations']['runs'], [decoded['observation']])
        self.assertIs(bundle['raw_runs'][0], decoded)
        self.assertTrue(bundle['comparison_ready'])
        self.assertEqual(bundle['projection']['gaps'], [])
        self.assertIn('not authenticated', bundle['evidence']['execution_receipt'])
        self.assertIn('blocked', bundle['evidence']['reliance'])
        json.dumps(bundle, sort_keys=True)
        twice = k.observation_bundle([decoded, run()], source_sha256=MAIN_K_SHA256)
        self.assertEqual(len(twice['observations']['programs']), 1)
        self.assertEqual(len(twice['observations']['runs']), 2)
        self.assertTrue(twice['comparison_ready'])

    def test_retained_run_input_projects_without_evaluation(self):
        retained = k.retained_run_input(RUN_INPUT)
        self.assertEqual({key: retained[key] for key in
                          ('code_k', 'entry_k', 'initial_fuel', 'stack_limit')},
                         dict(code_k='uiNoFuns()', entry_k=ENTRY, initial_fuel=1, stack_limit=8))
        self.assertEqual(retained['raw_text'], RUN_INPUT)
        self.assertEqual(retained['raw_sha256'], RUN_INPUT_RECORD[1])
        self.assertIsNone(run(run_input=None)['run_input'])

    # Synthetic hostile mutations of copies of the actual fixtures.

    def test_warning_severity_and_entry_admission_are_closed(self):
        raw = actual('05-exec-depth1-2/stderr')
        warning = b'kore-exec: Warning (WarnDepthLimitExceeded):'
        for old, new, gap in (
                (warning, b'kore-exec: Info (WarnDepthLimitExceeded):',
                 'log-entry-not-admitted-Info-WarnDepthLimitExceeded'),
                (warning, b'kore-exec: Error (WarnDepthLimitExceeded):',
                 'log-entry-not-admitted-Error-WarnDepthLimitExceeded'),
                (warning, b'kore-exec: Critical (WarnDepthLimitExceeded):',
                 'log-entry-not-admitted-Critical-WarnDepthLimitExceeded'),
                (warning, b'kore-exec: Warning (WarnDepthLimitExceeded2):',
                 'log-entry-not-admitted-Warning-WarnDepthLimitExceeded2'),
                (b'kore-exec: Info (InfoExecDepth):', b'kore-exec: Warning (InfoExecDepth):',
                 'log-entry-not-admitted-Warning-InfoExecDepth'),
                (b'kore-exec: Debug (DebugAppliedRewriteRules):',
                 b'kore-exec: Info (DebugAppliedRewriteRules):',
                 'log-entry-not-admitted-Info-DebugAppliedRewriteRules')):
            with self.subTest(gap=gap):
                self.assert_edge_rejected(replaced(raw, old, new), gap,
                                          'has-no-single-observed-source-rewrite')

    def test_depth_limit_and_exec_depth_guards(self):
        raw = actual('05-exec-depth1-2/stderr')
        layout = 'has-no-single-observed-source-rewrite'
        for name, mutated, gaps in (
                ('other-limit', replaced(raw, b'depth limit 1 was', b'depth limit 2 was'),
                 [layout]),
                ('missing-warning', replaced(raw, WARN_ENTRY, b''), [layout]),
                ('repeated-warning', replaced(raw, WARN_ENTRY, WARN_ENTRY * 2),
                 ['depth-limit-warning-repeated', layout]),
                ('inexact-warning', replaced(raw, b'was exceeded.\n', b'was exceeded!\n'),
                 ['depth-limit-warning-not-one-exact-line', layout]),
                ('depth-2', replaced(raw, b'exec depth: 1\n', b'exec depth: 2\n'), [layout]),
                ('depth-0', replaced(raw, b'exec depth: 1\n', b'exec depth: 0\n'), [layout]),
                ('depth-not-final', replaced(raw, WARN_ENTRY + DEPTH_1, DEPTH_1 + WARN_ENTRY),
                 ['execution-depth-not-single-final-entry', layout])):
            with self.subTest(case=name):
                self.assert_edge_rejected(mutated, *gaps)

    def test_declared_depth_must_be_exactly_one(self):
        for value in ('2', '01'):
            with self.subTest(depth=value):
                rows = captures()
                set_option(rows[1]['engine_receipt'], '--depth', value)
                result = run(rows)
                self.assertIn('edge-0-engine-receipt-argv-does-not-declare-exactly-depth-1',
                              result['gaps'])
                self.assertIn('edge-0-has-no-single-observed-source-rewrite', result['gaps'])
                self.assertIn('chain-engine-command-template-differs', result['gaps'])
                self.assertIsNone(result['observation']['records'][0]['rule_start_line'])

    def test_no_rule_terminal_guards(self):
        raw = actual('07-exec-depth1-4/stderr')
        layout = 'has-no-observed-no-rule-entry-at-depth-0'
        for name, mutated, gaps in (
                ('location-row', replaced(raw, NO_RULES, NO_RULES + location(b'80:8-81:45')),
                 ['no-rule-entry-has-location-rows']),
                ('warning', replaced(raw, DEPTH_0, WARN_ENTRY + DEPTH_0), [layout]),
                ('applied', replaced(raw, NO_RULES, APPLIED + location(b'80:8-81:45')), [layout]),
                ('no-variant', replaced(raw, NO_RULES, b''),
                 ['applied-rule-entry-has-no-complete-location-section', layout]),
                ('depth0-exec-log', actual('03-exec-depth0/stderr'),
                 [layout, 'logged-configuration-is-not-input-kore'])):
            with self.subTest(case=name):
                result = self.assert_terminal_rejected(relogged(probe(TERMINAL), mutated), *gaps)
                self.assertIsNone(result['observation']['terminal_has_next'])
                self.assertIn('terminal-has-next-not-supplied', result['gaps'])
        terminal = probe(TERMINAL)
        terminal['state_kore'] = actual('05-exec-depth1-2/state.kore')
        terminal['engine_receipt']['output_sha256'] = sha(terminal['state_kore'])
        self.assert_terminal_rejected(terminal, 'replay-changed-raw-kore')

    def test_terminal_depth_one_replay_is_observed_next(self):
        raw = replaced(actual('07-exec-depth1-4/stderr'), b'exec depth: 0\n', b'exec depth: 1\n')
        result = self.assert_terminal_rejected(relogged(probe(TERMINAL), raw),
                                               'replay-observed-a-rewrite',
                                               'has-no-observed-no-rule-entry-at-depth-0')
        self.assertIs(result['observation']['terminal_has_next'], True)
        self.assertIn('supplied-terminal-has-another-transition', result['gaps'])

    def test_rewrite_steps_reject_no_rule_and_depth0_logs(self):
        raw = actual('05-exec-depth1-2/stderr')
        layout = 'has-no-single-observed-source-rewrite'
        self.assert_edge_rejected(replaced(raw, APPLIED + location(b'102:8-104:41'), NO_RULES),
                                  layout)
        self.assert_edge_rejected(replaced(raw, location(b'102:8-104:41'), b''),
                                  'applied-rule-entry-has-no-location', layout)
        self.assert_edge_rejected(actual('07-exec-depth1-4/stderr'), layout,
                                  'logged-configuration-is-not-input-kore')
        self.assert_edge_rejected(actual('03-exec-depth0/stderr'), layout)
        result = self.assert_startup_rejected(
            relogged(probe(STARTUP), actual('03-exec-depth0/stderr')), layout)
        self.assertIsNone(result['startup_probe']['before_kore'])

    def test_logged_input_pairing_guards(self):
        raw = actual('05-exec-depth1-2/stderr')
        result = self.assert_edge_rejected(replaced(raw, b'("7")', b'("9")'))
        self.assertEqual(prefixed(result, 'edge-0-'),
                         ['edge-0-logged-configuration-is-not-input-kore'])
        rows = captures()
        log05, log06 = rows[1]['incoming_log'], rows[2]['incoming_log']
        relogged(rows[1], log06, 'incoming_log')
        relogged(rows[2], log05, 'incoming_log')
        result = run(rows)
        self.assertEqual(prefixed(result, 'edge-0-'),
                         ['edge-0-logged-configuration-is-not-input-kore'])
        self.assertEqual(prefixed(result, 'edge-1-'),
                         ['edge-1-logged-configuration-is-not-input-kore'])
        terminal = k.decode_terminal_probe(probe(TERMINAL),
                                           terminal_configuration=pretty('11-kprint-06'),
                                           terminal_state_kore=actual('05-exec-depth1-2/state.kore'),
                                           entry_k=ENTRY, source_path=SOURCE)
        self.assertIn('terminal-logged-configuration-is-not-input-kore', terminal['gaps'])
        self.assertIn('terminal-replay-changed-raw-kore', terminal['gaps'])
        self.assertIsNone(terminal['terminal_has_next'])
        terminal = k.decode_terminal_probe(probe(TERMINAL),
                                           terminal_configuration=pretty('11-kprint-06'),
                                           terminal_state_kore=None, entry_k=ENTRY,
                                           source_path=SOURCE)
        self.assertIn('terminal-input-kore-not-supplied', terminal['gaps'])
        self.assertIsNone(terminal['terminal_has_next'])
        result = run(startup_probe=None)
        self.assertIn('startup-probe-not-supplied', result['gaps'])
        self.assertIn('edge-0-input-kore-not-supplied', result['gaps'])
        self.assertEqual(prefixed(result, 'edge-1-'), [])
        self.assertIsNone(result['startup_probe'])
        self.assertIsNone(result['raw_startup_probe'])
        self.assertFalse(result['comparison_ready'])

    def test_print_pairing_guards(self):
        def other_input(rows):
            rows[1]['print_receipt']['argv'][2] = state_path('06-exec-depth1-3')

        def engine_hash(rows):
            rows[1]['print_receipt']['stdout_sha256'] = rows[1]['engine_receipt']['output_sha256']

        def other_stdout(rows):
            rows[1]['configuration'] = pretty('11-kprint-06')

        def coloured(rows):
            rows[1]['print_receipt']['argv'][3] = 'true'

        def short(rows):
            del rows[1]['print_receipt']['argv'][4]

        for mutate, gaps in (
                (other_input, ['edge-0-print-receipt-input-is-not-engine-output-path']),
                (engine_hash, ['edge-0-print-receipt-stdout-configuration-sha256-mismatch']),
                (other_stdout, ['edge-0-print-receipt-stdout-configuration-sha256-mismatch']),
                (coloured, ['edge-0-print-receipt-argv-not-uncoloured-unfiltered-print',
                            'chain-print-command-template-differs']),
                (short, ['edge-0-print-receipt-argv-not-uncoloured-unfiltered-print',
                         'edge-0-print-receipt-input-is-not-engine-output-path',
                         'chain-print-command-template-differs'])):
            with self.subTest(mutation=mutate.__name__):
                rows = captures()
                mutate(rows)
                result = run(rows)
                self.assertFalse(result['comparison_ready'])
                for gap in gaps:
                    self.assertIn(gap, result['gaps'])
                self.assertIsNone(result['observation']['records'][0]['rule_start_line'])

    def test_path_chain_guards(self):
        rows = captures()
        set_option(rows[2]['engine_receipt'], '--pattern', state_path('04-exec-depth1-1'))
        result = run(rows)
        self.assertIn('chain-edge-1-pattern-is-not-edge-0-output', result['gaps'])
        self.assertFalse(result['comparison_ready'])

        rows = captures()
        set_option(rows[2]['engine_receipt'], '--output', state_path('05-exec-depth1-2'))
        result = run(rows)
        for gap in ('chain-engine-output-paths-not-distinct',
                    'chain-terminal-pattern-is-not-edge-1-output',
                    'edge-1-engine-receipt-argv-not-distinct-pattern-and-output',
                    'edge-1-print-receipt-input-is-not-engine-output-path'):
            self.assertIn(gap, result['gaps'])

        terminal = probe(TERMINAL)
        set_option(terminal['engine_receipt'], '--pattern', state_path('05-exec-depth1-2'))
        result = run(terminal_probe=terminal)
        self.assertIn('chain-terminal-pattern-is-not-edge-1-output', result['gaps'])
        self.assertIs(result['observation']['terminal_has_next'], False)
        self.assertFalse(result['comparison_ready'])

        startup = probe(STARTUP)
        set_option(startup['engine_receipt'], '--output', ROOT + '/elsewhere/state.kore')
        result = run(startup_probe=startup)
        self.assertIn('chain-edge-0-pattern-is-not-startup-output', result['gaps'])
        self.assertIn('startup-print-receipt-input-is-not-engine-output-path', result['gaps'])

        terminal = probe(TERMINAL)
        set_option(terminal['engine_receipt'], '--smt', 'z3')
        result = run(terminal_probe=terminal)
        self.assertEqual(result['gaps'], ['chain-engine-command-template-differs'])
        self.assertIs(result['observation']['terminal_has_next'], False)

        rows = captures()
        set_option(rows[1]['engine_receipt'], '--module', 'MORIARTY-UI-SYNTAX')
        self.assertIn('chain-engine-command-template-differs', run(rows)['gaps'])

        rows = captures()
        rows[2]['print_receipt']['argv'][1] = ROOT + '/other-kompiled'
        result = run(rows)
        self.assertEqual(result['gaps'], ['chain-print-command-template-differs'])

    def test_engine_option_set_is_closed(self):
        def extra(argv):
            argv.extend(['--strategy', 'all'])

        def no_timestamps(argv):
            argv.remove('--disable-log-timestamps')

        def json_log(argv):
            argv[argv.index('--log-format') + 1] = 'json'

        def more_entries(argv):
            argv[argv.index('--log-entries') + 1] = ('DebugAppliedRewriteRules,InfoExecDepth,'
                                                     'DebugAttemptedRewriteRules')

        def repeated_depth(argv):
            argv.extend(['--depth', '1'])

        def dash_value(argv):
            argv[argv.index('--depth') + 1] = '-1'

        def joined(argv):
            index = argv.index('--depth')
            argv[index:index + 2] = ['--depth=1']

        def option_definition(argv):
            argv[1] = '--definition'

        def no_pattern(argv):
            index = argv.index('--pattern')
            del argv[index:index + 2]

        closed = ['engine-receipt-argv-not-admitted-option-set',
                  'engine-receipt-argv-does-not-declare-exactly-depth-1']
        logging = ['engine-receipt-argv-not-standard-untimestamped-selected-log']
        for mutate, gaps in ((extra, closed), (no_timestamps, logging), (json_log, logging),
                             (more_entries, logging), (repeated_depth, closed),
                             (dash_value, closed), (joined, closed), (option_definition, closed),
                             (no_pattern, ['engine-receipt-argv-not-distinct-pattern-and-output'])):
            with self.subTest(mutation=mutate.__name__):
                rows = captures()
                mutate(rows[1]['engine_receipt']['argv'])
                result = run(rows)
                self.assertFalse(result['comparison_ready'])
                for gap in gaps:
                    self.assertIn('edge-0-' + gap, result['gaps'])
                self.assertIsNone(result['observation']['records'][0]['rule_start_line'])

    def test_terminal_argv_must_declare_exactly_depth_one(self):
        def depth_zero(argv):
            argv[argv.index('--depth') + 1] = '0'

        def joined(argv):
            index = argv.index('--depth')
            argv[index:index + 2] = ['--depth=1']

        def repeated(argv):
            argv.extend(['--depth', '0'])

        def bare(argv):
            del argv[1:]

        for mutate in (depth_zero, joined, repeated, bare):
            with self.subTest(mutation=mutate.__name__):
                terminal = probe(TERMINAL)
                mutate(terminal['engine_receipt']['argv'])
                result = self.assert_terminal_rejected(
                    terminal, 'engine-receipt-argv-does-not-declare-exactly-depth-1')
                self.assertIsNone(result['observation']['terminal_has_next'])

    def test_receipt_consistency_blocks_edge_terminal_and_startup(self):
        for receipt, field, value, gap in (
                ('engine_receipt', 'exit_status', 1, 'engine-receipt-exit-status-nonzero'),
                ('engine_receipt', 'exit_status', -9, 'engine-receipt-exit-status-nonzero'),
                ('engine_receipt', 'stderr_sha256', OTHER,
                 'engine-receipt-stderr-log-sha256-mismatch'),
                ('engine_receipt', 'stdout_sha256', OTHER, 'engine-receipt-stdout-not-empty'),
                ('engine_receipt', 'output_sha256', OTHER,
                 'engine-receipt-output-kore-sha256-mismatch'),
                ('print_receipt', 'exit_status', 1, 'print-receipt-exit-status-nonzero'),
                ('print_receipt', 'stdout_sha256', OTHER,
                 'print-receipt-stdout-configuration-sha256-mismatch'),
                ('print_receipt', 'stderr_sha256', OTHER, 'print-receipt-stderr-not-empty')):
            with self.subTest(gap=gap, value=value, probe='edge'):
                rows = captures()
                rows[1][receipt][field] = value
                self.assert_edge_rejected(rows[1]['incoming_log'], gap) if False else None
                result = run(rows)
                self.assertIn('edge-0-' + gap, result['gaps'])
                self.assertEqual([row['rule_start_line'] for row in
                                  result['observation']['records']], [None, None])
            with self.subTest(gap=gap, value=value, probe='terminal'):
                terminal = probe(TERMINAL)
                terminal[receipt][field] = value
                result = self.assert_terminal_rejected(terminal, gap)
                self.assertIsNone(result['observation']['terminal_has_next'])
            with self.subTest(gap=gap, value=value, probe='startup'):
                startup = probe(STARTUP)
                startup[receipt][field] = value
                self.assert_startup_rejected(startup, gap)

    def test_receipt_hash_over_original_bytes_detects_normalization(self):
        terminal = probe(TERMINAL)
        terminal['engine_receipt']['stderr_sha256'] = sha(terminal['log'].replace(b'\n', b'\r\n'))
        result = k.decode_terminal_probe(terminal, terminal_configuration=pretty('11-kprint-06'),
                                        terminal_state_kore=actual('06-exec-depth1-3/state.kore'),
                                        entry_k=ENTRY, source_path=SOURCE)
        self.assertIsNone(result['terminal_has_next'])
        self.assertIn('terminal-engine-receipt-stderr-log-sha256-mismatch', result['gaps'])

    def test_missing_receipts_are_gaps(self):
        for receipt, gap in (('engine_receipt', 'engine-receipt-not-supplied'),
                             ('print_receipt', 'print-receipt-not-supplied')):
            with self.subTest(gap=gap):
                rows = captures()
                del rows[1][receipt]
                result = run(rows)
                self.assertIn('edge-0-' + gap, result['gaps'])
                self.assertIsNone(result['observation']['records'][0]['rule_start_line'])
                terminal = probe(TERMINAL)
                del terminal[receipt]
                self.assertIsNone(self.assert_terminal_rejected(terminal, gap)
                                  ['observation']['terminal_has_next'])
                startup = probe(STARTUP)
                del startup[receipt]
                self.assert_startup_rejected(startup, gap)

    def test_full_span_identity_guards(self):
        raw = actual('05-exec-depth1-2/stderr')
        row = location(b'102:8-104:41')
        for span in (b'102:8-104:40', b'102:9-104:41', b'103:8-104:41', b'102:8-105:41'):
            with self.subTest(span=span):
                result = self.assert_edge_rejected(replaced(raw, row, location(span)))
                self.assertEqual(prefixed(result, 'edge-0-'),
                                 ['edge-0-applied-rule-span-not-in-compiled-inventory'])
        spans = inventory()
        spans[5]['end_column'] = 40
        result = run(rule_spans=spans)
        self.assertEqual(prefixed(result, 'edge-0-'),
                         ['edge-0-applied-rule-span-not-in-compiled-inventory'])
        result = run(rule_spans=inventory()[1:])
        self.assertIn('startup-applied-rule-span-not-in-compiled-inventory', result['gaps'])
        self.assertIsNone(result['startup_probe']['rule'])
        decoy = inventory() + [dict(source_path=SOURCE, start_line=102, start_column=8,
                                    end_line=104, end_column=40, unique_id=sha(b'decoy'))]
        result = run(rule_spans=decoy)
        self.assertEqual(result['gaps'], [])
        self.assertEqual(result['observed_rule_identities'][0]['unique_id'], INVENTORY[5][4])
        self.assert_edge_rejected(replaced(raw, row, location(b'104:41-102:8')),
                                  'applied-rule-location-span-reversed',
                                  'has-no-single-observed-source-rewrite')
        self.assert_edge_rejected(replaced(raw, row, location(b'0:8-104:41')),
                                  'applied-rule-location-outside-selected-source-or-format')
        result = run(rule_spans=None)
        for gap in ('rule-span-inventory-not-supplied', 'edge-0-rule-span-inventory-not-supplied',
                    'edge-1-rule-span-inventory-not-supplied',
                    'startup-rule-span-inventory-not-supplied'):
            self.assertIn(gap, result['gaps'])
        self.assertEqual(result['observed_rule_identities'], [None, None])
        self.assertIsNone(result['startup_probe']['rule'])

    def test_inventory_validation_raises(self):
        def changed(**change):
            spans = inventory()
            spans[5].update(change)
            return spans

        duplicate_id = inventory()
        duplicate_id.append(dict(duplicate_id[0], start_line=1, end_line=1))
        missing = inventory()
        del missing[0]['unique_id']
        for name, spans in (
                ('empty', []), ('tuple', tuple(inventory())), ('missing-key', missing),
                ('other-source', inventory('/other/main.k')),
                ('duplicate-span', inventory() + [inventory()[0]]),
                ('duplicate-id', duplicate_id), ('reversed', changed(start_line=105)),
                ('zero-column', changed(start_column=0)), ('boolean', changed(end_line=True)),
                ('text-line', changed(start_line='102')),
                ('uppercase-id', changed(unique_id=INVENTORY[5][4].upper())),
                ('oversized', [inventory()[0]] * (k.MAX_RULE_SPANS + 1))):
            with self.subTest(case=name):
                with self.assertRaises(ValueError):
                    run(rule_spans=spans)

    def test_size_bounds(self):
        for raw in (b' \n' * (k.MAX_LOG_BYTES // 2 + 1), b'\n' * (k.MAX_LOG_LINES + 1)):
            decoded = k.decode_log(raw, source_path=SOURCE)
            self.assertEqual(decoded['gaps'], ['log-exceeds-decoder-size-bound'])
        self.assert_edge_rejected(actual('05-exec-depth1-2/stderr')
                                  + b' \n' * (k.MAX_LOG_BYTES // 2),
                                  'log-exceeds-decoder-size-bound',
                                  'has-no-single-observed-source-rewrite')
        rows = captures()
        rows[1]['state_kore'] += b'\n' * k.MAX_KORE_BYTES
        rows[1]['engine_receipt']['output_sha256'] = sha(rows[1]['state_kore'])
        result = run(rows)
        self.assertIn('edge-0-state-kore-exceeds-retention-bound', result['gaps'])
        self.assertIn('edge-1-logged-configuration-is-not-input-kore', result['gaps'])
        rows = captures()
        rows[1]['configuration'] = replaced(rows[1]['configuration'], '    uiPending ( )\n',
                                            '    uiPending ( )'
                                            + ' ' * k.MAX_CONFIGURATION_BYTES + '\n')
        with self.assertRaisesRegex(ValueError, 'size bound'):
            run(rows)
        with self.assertRaises(ValueError):
            k.decode_capture('N01', ENTRY + ' ' * k.MAX_TERM_BYTES, captures(),
                             source_path=SOURCE)
        with self.assertRaises(ValueError):
            k.decode_capture('N' * 65, ENTRY, captures(), source_path=SOURCE)
        with self.assertRaises(ValueError):
            run(run_input=RUN_INPUT + ' ' * k.MAX_TERM_BYTES)
        with self.assertRaises(ValueError):
            run([dict(configuration=pretty('09-kprint-04'))
                 for _ in range(k.MAX_CAPTURES + 1)])
        for argv in (['x'] * (k.MAX_ARGV + 1), [KORE_EXEC, 'x' * (k.MAX_ARG_CHARS + 1)]):
            rows = captures()
            rows[1]['engine_receipt']['argv'] = argv
            with self.assertRaises(ValueError):
                run(rows)

    def test_nesting_bounds_precede_shared_decoders(self):
        text = pretty('09-kprint-04')
        out = '    uiPending ( )\n'

        def nested(depth, opener='( ', closer=' )'):
            return replaced(text, out, '    ' + opener * depth + 'uiPending ( )'
                            + closer * depth + '\n')

        result = k.decode_capture('N01', ENTRY, [dict(configuration=nested(k.MAX_NESTING - 1))],
                                  source_path=SOURCE)
        self.assertEqual(result['observation']['initial']['output_k'], 'uiPending()')
        for configuration in (nested(k.MAX_NESTING), nested(3000, '(', ')'),
                              replaced(text, out, '    uiPending ( ) )\n'),
                              replaced(text, out, '    ( uiPending ( )\n')):
            with self.subTest(size=len(configuration)):
                with self.assertRaises(ValueError):
                    k.decode_capture('N01', ENTRY, [dict(configuration=configuration)],
                                     source_path=SOURCE)
        deep_entry = '(' * 2000 + ENTRY + ')' * 2000
        with self.assertRaisesRegex(ValueError, 'nesting'):
            k.decode_capture('N01', deep_entry, captures(), source_path=SOURCE)
        with self.assertRaisesRegex(ValueError, 'nesting'):
            k.decode_terminal_probe(probe(TERMINAL), terminal_configuration=pretty('11-kprint-06'),
                                    terminal_state_kore=actual('06-exec-depth1-3/state.kore'),
                                    entry_k=deep_entry, source_path=SOURCE)
        with self.assertRaisesRegex(ValueError, 'nesting'):
            run(run_input='uiRun(uiNoFuns(), ' + deep_entry + ', 1, 8)')

    def test_unit_tail_hostile_variants_raise(self):
        line = '    uiRet ( uiU ( 7 ) ) ~> .K\n'
        for variant in ('    uiRet ( uiU ( 7 ) ) ~> uiRet ( uiU ( 7 ) ) ~> .K\n',
                        '    uiRet ( uiU ( 7 ) ) ~> uiPending ( )\n',
                        '    uiRet ( uiU ( 7 ) ) ~> .K ~> .K\n', '    .K ~> .K\n',
                        '    uiRet ( uiU ( 7 ) )~>.K\n', '    uiRet ( uiU ( 7 ) )  ~>  .K\n',
                        '    uiRet ( uiU ( 7 ) ) ~> .KK\n', '    ~> .K\n'):
            with self.subTest(variant=variant):
                rows = captures()
                rows[1]['configuration'] = replaced(rows[1]['configuration'], line, variant)
                with self.assertRaisesRegex(ValueError, 'actual-format gap: K sequence'):
                    run(rows)
        rows = captures()
        rows[1]['configuration'] = replaced(rows[1]['configuration'], '  </k>\n',
                                            '  </k>\n  <k>\n    .K\n  </k>\n')
        with self.assertRaisesRegex(ValueError, 'actual-format gap: K sequence'):
            run(rows)
        rows = captures()
        rows[2]['configuration'] = replaced(rows[2]['configuration'], '    uiDone ( uiU ( 7 ) )\n',
                                            '    uiDone ( uiU ( 7 ) ) ~> .K\n')
        with self.assertRaisesRegex(ValueError, 'actual-format gap: K sequence'):
            run(rows)
        terminal = probe(TERMINAL)
        terminal['configuration'] = replaced(terminal['configuration'], '    .K\n',
                                             '    uiRet ( uiU ( 7 ) )~>.K\n')
        with self.assertRaisesRegex(ValueError, 'actual-format gap: K sequence'):
            run(terminal_probe=terminal)
        startup = probe(STARTUP)
        startup['configuration'] = replaced(startup['configuration'], ' ~> .K\n',
                                            ' ~> uiPending ( )\n')
        with self.assertRaisesRegex(ValueError, 'actual-format gap: K sequence'):
            run(startup_probe=startup)

    def test_unknown_or_extra_cells_fail_closed(self):
        counter = '  <generatedCounter>\n    0\n  </generatedCounter>\n'
        for cell, inserted in (('generatedCounter', counter),
                               ('foo', '  <foo>\n    0\n  </foo>\n')):
            with self.subTest(cell=cell):
                rows = captures()
                reprinted(rows[2], replaced(rows[2]['configuration'], '</generatedTop>\n',
                                            inserted + '</generatedTop>\n'))
                with self.assertRaisesRegex(ValueError, 'actual-format gap: cell <' + cell):
                    run(rows)
        terminal = probe(TERMINAL)
        reprinted(terminal, replaced(terminal['configuration'], '</generatedTop>\n',
                                     counter + '</generatedTop>\n'))
        with self.assertRaisesRegex(ValueError, 'actual-format gap: cell <generatedCounter>'):
            run(terminal_probe=terminal)
        startup = probe(STARTUP)
        reprinted(startup, replaced(startup['configuration'], '</generatedTop>\n',
                                    counter + '</generatedTop>\n'))
        with self.assertRaisesRegex(ValueError, 'actual-format gap: cell <generatedCounter>'):
            run(startup_probe=startup)
        rows = captures()
        reprinted(rows[2], replaced(rows[2]['configuration'], '  <env>\n    .Map\n  </env>\n', ''))
        with self.assertRaises(ValueError):
            run(rows)

    def test_log_framing_hostile_variants(self):
        raw = actual('05-exec-depth1-2/stderr')
        row = location(b'102:8-104:41')
        builtin = (b'/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-'
                   b'4a46d1231473b599c699160132fd6e76a5c46406/include/kframework/builtin/domains.md')
        foreign = raw.replace(b'kore-exec:', b'kore-print:', 1)
        self.assertNotEqual(foreign, raw)
        for name, mutated, gaps in (
                ('error-entry', replaced(raw, WARN_ENTRY, b'kore-exec: Error (ErrorException):\n'
                                         b'    synthetic failure\n' + WARN_ENTRY),
                 ['log-entry-not-admitted-Error-ErrorException']),
                ('preamble', b'synthetic preamble\n' + raw,
                 ['log-has-unclaimed-top-level-text']),
                ('foreign-header', foreign, ['log-has-unclaimed-top-level-text',
                                             'log-has-text-before-first-entry']),
                ('timestamp', replaced(raw, b'kore-exec: Info (InfoExecDepth):',
                                       b'kore-exec: [123] Info (InfoExecDepth):'),
                 ['log-has-unclaimed-top-level-text']),
                ('bak-path', replaced(raw, row, b'    ' + SOURCE.encode() + b'.bak:102:8-104:41\n'),
                 ['applied-rule-location-outside-selected-source-or-format']),
                ('prefixed-path', replaced(raw, row, b'    x' + SOURCE.encode()
                                           + b':102:8-104:41\n'),
                 ['applied-rule-location-outside-selected-source-or-format']),
                ('builtin-path', replaced(raw, row, b'    ' + builtin + b':102:8-104:41\n'),
                 ['applied-rule-location-outside-selected-source-or-format']),
                ('trailing-text', replaced(raw, row, row[:-1] + b' x\n'),
                 ['applied-rule-location-outside-selected-source-or-format']),
                ('homoglyph-path', replaced(raw, row, b'    '
                                            + SOURCE.replace('i', '\u0456', 1).encode('utf-8')
                                            + b':102:8-104:41\n'), ['log-not-ascii']),
                ('two-locations', replaced(raw, row, row + location(b'80:8-81:45')),
                 ['has-no-single-observed-source-rewrite']),
                ('duplicate-depth', replaced(raw, DEPTH_1, DEPTH_1 * 2),
                 ['execution-depth-not-single-final-entry']),
                ('split-depth', replaced(raw, b'    execution complete: exec depth: 1\n',
                                         b'    execution complete:\n\n    exec depth: 1\n'),
                 ['log-has-blank-line', 'execution-depth-entry-not-one-exact-line']),
                ('crlf', raw.replace(b'\n', b'\r\n'), ['log-contains-control-character']),
                ('tab', replaced(raw, b'    execution complete', b'\texecution complete'),
                 ['log-contains-control-character']),
                ('unterminated', raw[:-1], ['log-empty-or-not-newline-terminated']),
                ('empty', b'', ['log-empty-or-not-newline-terminated']),
                ('injected-header', replaced(raw, b'          /* term: */\n',
                                             b'kore-exec: Info (InfoExecDepth):\n'
                                             b'          /* term: */\n'),
                 ['execution-depth-entry-not-one-exact-line']),
                ('no-on-configuration', replaced(raw, b'    On configuration:\n', b''),
                 ['applied-rule-entry-has-no-complete-location-section',
                  'has-no-single-observed-source-rewrite']),
                ('unindented-kore', replaced(raw, b'\n      ))\n', b'\n    ))\n'),
                 ['applied-rule-configuration-not-indented-kore-text'])):
            with self.subTest(case=name):
                self.assert_edge_rejected(mutated, *gaps)

    def test_whitespace_floods_finish_within_time_bound(self):
        flood = (b' ' * 60 + b'\n') * 65000
        self.assertLess(len(flood), k.MAX_LOG_BYTES)
        for raw, gap in ((b' \n' * 5_000_000, 'log-exceeds-decoder-size-bound'),
                         (flood, 'log-has-text-before-first-entry')):
            started = time.monotonic()
            result = k.decode_log(raw, source_path=SOURCE)
            self.assertLess(time.monotonic() - started, 5.0)
            self.assertIn(gap, result['gaps'])
            self.assertEqual(result['execution_depths'], [])

    def test_malformed_inputs_raise_value_error(self):
        def missing_log(rows):
            del rows[1]['incoming_log']

        def text_log(rows):
            rows[1]['incoming_log'] = rows[1]['incoming_log'].decode('ascii')

        def text_kore(rows):
            rows[1]['state_kore'] = rows[1]['state_kore'].decode('ascii')

        def bytes_configuration(rows):
            rows[1]['configuration'] = rows[1]['configuration'].encode('ascii')

        def boolean_status(rows):
            rows[1]['engine_receipt']['exit_status'] = True

        def text_status(rows):
            rows[1]['print_receipt']['exit_status'] = '0'

        def uppercase_hash(rows):
            receipt = rows[1]['engine_receipt']
            receipt['stderr_sha256'] = receipt['stderr_sha256'].upper()

        def short_hash(rows):
            rows[1]['engine_receipt']['output_sha256'] = 'ab'

        def extra_capture_key(rows):
            rows[1]['extra'] = 'x'

        def first_log(rows):
            rows[0]['incoming_log'] = rows[1]['incoming_log']

        def first_receipt(rows):
            rows[0]['engine_receipt'] = rows[1]['engine_receipt']

        def extra_receipt_key(rows):
            rows[1]['print_receipt']['output_sha256'] = EMPTY

        def missing_receipt_key(rows):
            del rows[1]['engine_receipt']['output_sha256']

        def empty_argv(rows):
            rows[1]['engine_receipt']['argv'] = []

        def tuple_argv(rows):
            rows[1]['engine_receipt']['argv'] = tuple(rows[1]['engine_receipt']['argv'])

        def empty_arg(rows):
            rows[1]['print_receipt']['argv'][3] = ''

        def nul_arg(rows):
            rows[1]['engine_receipt']['argv'][3] = 'MORIARTY-UI\x00'

        def receipt_list(rows):
            rows[1]['engine_receipt'] = list(rows[1]['engine_receipt'].items())

        def capture_list(rows):
            rows[1] = list(rows[1].items())

        for mutate in (missing_log, text_log, text_kore, bytes_configuration, boolean_status,
                       text_status, uppercase_hash, short_hash, extra_capture_key, first_log,
                       first_receipt, extra_receipt_key, missing_receipt_key, empty_argv,
                       tuple_argv, empty_arg, nul_arg, receipt_list, capture_list):
            with self.subTest(mutation=mutate.__name__):
                rows = captures()
                mutate(rows)
                with self.assertRaises(ValueError):
                    run(rows)

        text_terminal = probe(TERMINAL)
        text_terminal['log'] = text_terminal['log'].decode('ascii')
        extra_terminal = probe(TERMINAL)
        extra_terminal['incoming_log'] = extra_terminal['log']
        status_terminal = probe(TERMINAL)
        status_terminal['engine_receipt']['exit_status'] = False
        partial_startup = probe(STARTUP)
        del partial_startup['state_kore']
        for name, overrides in (
                ('terminal-text-log', dict(terminal_probe=text_terminal)),
                ('terminal-extra-key', dict(terminal_probe=extra_terminal)),
                ('terminal-boolean-status', dict(terminal_probe=status_terminal)),
                ('startup-missing-kore', dict(startup_probe=partial_startup)),
                ('startup-list', dict(startup_probe=list(probe(STARTUP).items()))),
                ('relative-source', dict(source_path='compile-input/models/sherlock/k/main.k')),
                ('spaced-source', dict(source_path=SOURCE + ' copy')),
                ('path-object-source', dict(source_path=Path(SOURCE)))):
            with self.subTest(case=name):
                with self.assertRaises(ValueError):
                    run(**overrides)
        for rows in (tuple(captures()), []):
            with self.assertRaises(ValueError):
                run(rows)
        for case_id in ('', 7):
            with self.assertRaises(ValueError):
                k.decode_capture(case_id, ENTRY, captures(), source_path=SOURCE)
        with self.assertRaises(ValueError):
            k.decode_capture('N01', None, captures(), source_path=SOURCE)
        raw = actual('05-exec-depth1-2/stderr')
        for value, source in ((raw.decode('ascii'), SOURCE), (bytearray(raw), SOURCE),
                              (raw, 'main.k')):
            with self.assertRaises(ValueError):
                k.decode_log(value, source_path=source)

    def test_bundle_guards(self):
        decoded = run()
        for value in ('declared-source-sha', MAIN_K_SHA256.upper(), MAIN_K_SHA256[:-1], None):
            with self.subTest(source_sha256=value):
                with self.assertRaises(ValueError):
                    k.observation_bundle([decoded], source_sha256=value)
        for runs in ([], (decoded,), [decoded['observation']], [dict(decoded, format='other')]):
            with self.assertRaises(ValueError):
                k.observation_bundle(runs, source_sha256=MAIN_K_SHA256)
        gapped = run(startup_probe=None)
        bundle = k.observation_bundle([decoded, gapped], source_sha256=MAIN_K_SHA256)
        self.assertFalse(bundle['comparison_ready'])
        self.assertIn('run-1-startup-probe-not-supplied', bundle['projection']['gaps'])
        self.assertEqual(len(bundle['observations']['programs']), 1)
        moved = run(source_path='/other/main.k', rule_spans=None)
        self.assertFalse(moved['comparison_ready'])
        bundle = k.observation_bundle([decoded, moved], source_sha256=MAIN_K_SHA256)
        self.assertFalse(bundle['comparison_ready'])
        self.assertIn('runs-source-path-differs', bundle['projection']['gaps'])

    def test_run_input_guards(self):
        for text in ('uiLiteral(uiU(7))', 'uiRun(uiNoFuns(), uiLiteral(uiU(7)), uiNil(), 8)',
                     'uiRun(uiNoFuns(), uiLiteral(uiU(7)), 1)', 'uiRun(uiNoFuns(),\tuiNil(), 1, 8)',
                     None if False else 7):
            with self.subTest(run_input=text):
                with self.assertRaises(ValueError):
                    k.retained_run_input(text)
        for text, gap in (
                ('uiRun(uiNoFuns(), uiLiteral(uiU(7)), 2, 8)\n',
                 'run-input-initial_fuel-differs-from-post-startup-initial'),
                ('uiRun(uiNoFuns(), uiLiteral(uiU(7)), 1, 9)\n',
                 'run-input-stack_limit-differs-from-post-startup-initial'),
                ('uiRun(uiNoFuns(), uiLiteral(uiU(8)), 1, 8)\n',
                 'run-input-entry_k-differs-from-post-startup-initial')):
            with self.subTest(gap=gap):
                result = run(run_input=text)
                self.assertFalse(result['comparison_ready'])
                self.assertEqual(result['gaps'], [gap])
        result = k.decode_capture('N01', 'uiLiteral(uiU(8))', captures(), source_path=SOURCE,
                                  rule_spans=inventory(), terminal_probe=probe(TERMINAL),
                                  startup_probe=probe(STARTUP), run_input=RUN_INPUT)
        self.assertIn('post-startup-initial-configuration-not-observed', result['gaps'])
        self.assertIn('run-input-entry_k-differs-from-post-startup-initial', result['gaps'])
        self.assertFalse(result['comparison_ready'])


if __name__ == '__main__':
    unittest.main()
