from __future__ import annotations

import math
import sys
import unicodedata
from pathlib import Path

from pypdf import PdfReader


def main() -> int:
    if len(sys.argv) != 2:
        raise SystemExit("usage: verify_pdf.py <pdf-path>")

    pdf_path = Path(sys.argv[1])
    if not pdf_path.is_file() or pdf_path.stat().st_size < 500:
        raise SystemExit("PDF output is missing or unexpectedly small")

    reader = PdfReader(str(pdf_path), strict=True)
    if len(reader.pages) != 1:
        raise SystemExit(f"expected exactly one A4 page; got {len(reader.pages)}")

    page = reader.pages[0]
    expected_width = 210.0 / 25.4 * 72.0
    expected_height = 297.0 / 25.4 * 72.0
    width = float(page.mediabox.width)
    height = float(page.mediabox.height)
    if not math.isclose(width, expected_width, abs_tol=2.0):
        raise SystemExit(f"A4 width mismatch: {width:.2f}pt")
    if not math.isclose(height, expected_height, abs_tol=2.0):
        raise SystemExit(f"A4 height mismatch: {height:.2f}pt")

    text = " ".join((page.extract_text() or "").split())
    text = unicodedata.normalize("NFC", text)
    expected_fragments = (
        "DCMS Phase 1 Windows Proof",
        "বাংলা পরীক্ষা",
        "দাঁতের যত্ন",
        "৳",
        "১,২৫০.০০",
    )
    missing = [fragment for fragment in expected_fragments if fragment not in text]
    if missing:
        raise SystemExit(f"PDF text extraction is missing expected content: {missing!r}")

    print(
        "PDF proof passed: 1 page, A4 media box "
        f"{width:.2f} x {height:.2f} pt, English/Bengali/currency text extracted."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
