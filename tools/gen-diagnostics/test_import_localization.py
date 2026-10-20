"""Source-32 import-localization behavior, matching upstream's Go cases."""

from pathlib import Path
import tempfile
import unittest
import xml.etree.ElementTree as ET

from import_localization import import_localization, read_messages


class ImportLocalizationTests(unittest.TestCase):
    def test_preserves_pending_and_filters_obsolete_translations(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            directory = Path(temp)
            source = directory / "source.json"
            handback = directory / "handback.json"
            output = directory / "loc/ko-KR.generated.json"
            source.write_text('''{
                "Updated": "Current English.",
                "New": "New English.",
                "StaleEnglish": "Changed English.",
                "Translated": "Source [text].",
                "SameAsEnglish": "English.",
                "ExistingEnglish": "Existing English.",
                "ExistingOnly": "Existing source."
            }''', encoding="utf-8")
            handback.write_text('''{
                "Updated": "Current English.",
                "New": "New English.",
                "StaleEnglish": "Old English.",
                "Translated": "새 번역]",
                "SameAsEnglish": "English.",
                "Removed": "Obsolete translation."
            }''', encoding="utf-8")
            Path(f"{handback}.lct").write_text('''<?xml version="1.0"?>
                <LCX xmlns="http://schemas.microsoft.com/locstudio/2006/6/lcx">
                  <Item ItemId=";String Table"><Item ItemId=";Strings">
                    <Item ItemId=";Updated"><Str><Tgt Stat="Update"/></Str></Item>
                    <Item ItemId=";New"><Str><Tgt/></Str></Item>
                    <Item ItemId=";StaleEnglish"><Str><Tgt/></Str></Item>
                    <Item ItemId=";Translated"><Str><Tgt Stat="Loc"/></Str></Item>
                    <Item ItemId=";SameAsEnglish"><Str><Tgt Stat="Loc"/></Str></Item>
                  </Item></Item>
                </LCX>''', encoding="utf-8")
            output.parent.mkdir()
            output.write_text('''{
                "Updated": "이전 번역",
                "StaleEnglish": "기존 번역",
                "Translated": "Previous translation.",
                "SameAsEnglish": "Previous translation.",
                "ExistingEnglish": "Existing English.",
                "ExistingOnly": "Keep this translation.",
                "Removed": "Remove this translation."
            }''', encoding="utf-8")

            import_localization(source, handback, output)
            expected = ('{\r\n'
                '  "ExistingOnly": "Keep this translation.",\r\n'
                '  "StaleEnglish": "기존 번역",\r\n'
                '  "Translated": "새 번역]",\r\n'
                '  "Updated": "이전 번역"\r\n'
                '}').encode()
            self.assertEqual(output.read_bytes(), expected)
            import_localization(source, handback, output)
            self.assertEqual(output.read_bytes(), expected)

            fresh = directory / "new-locale/generated.json"
            import_localization(source, handback, fresh)
            self.assertEqual(read_messages(fresh), {"Translated": "새 번역]"})

    def test_rejects_bad_metadata_without_changing_output(self) -> None:
        cases = {
            "missing resource": ("<LCX/>", "missing localization metadata"),
            "missing target": ('<LCX><Item ItemId=";Key"><Str/></Item></LCX>', "missing localization metadata"),
            "malformed XML": ("<LCX>", "no element found"),
            "unknown status": ('<LCX><Item ItemId=";Key"><Str><Tgt Stat="Unexpected"/></Str></Item></LCX>', "unknown localization status"),
            "unconfirmed New status": ('<LCX><Item ItemId=";Key"><Str><Tgt Stat="New"/></Str></Item></LCX>', "unknown localization status"),
            "duplicate resource": ('<LCX><Item ItemId=";Key"><Str/></Item><Item ItemId=";Key"><Str/></Item></LCX>', "duplicate resource ID"),
            "invalid resource ID": ('<LCX><Item ItemId="Key"><Str/></Item></LCX>', "invalid resource ID"),
        }
        for name, (metadata, error) in cases.items():
            with self.subTest(name=name), tempfile.TemporaryDirectory() as temp:
                directory = Path(temp)
                source = directory / "source.json"
                handback = directory / "handback.json"
                output = directory / "output.json"
                source.write_bytes(b'\xef\xbb\xbf{"Key":"English."}')
                handback.write_text('{"Key":"Translation."}', encoding="utf-8")
                Path(f"{handback}.lct").write_text(metadata, encoding="utf-8")
                existing = b'{"Key":"Existing translation."}'
                output.write_bytes(existing)
                with self.assertRaisesRegex((ValueError, ET.ParseError), error):
                    import_localization(source, handback, output)
                self.assertEqual(output.read_bytes(), existing)


if __name__ == "__main__":
    unittest.main()
