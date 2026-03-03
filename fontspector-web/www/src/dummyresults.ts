import { CheckResult } from "./types";

export const DummyResults: CheckResult[] = [
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/font_names",
    check_name: "Check font names are correct",
    check_rationale:
      "Google Fonts has several rules which need to be adhered to when setting a font's name table. Please read: https://googlefonts.github.io/gf-guide/statics.html#supported-styles https://googlefonts.github.io/gf-guide/statics.html#style-linking https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles https://googlefonts.github.io/gf-guide/statics.html#single-weight-families",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "Font names are incorrect:\n\n| Name                       | Current                                          | Expected                                     |\n|----------------------------|--------------------------------------------------|----------------------------------------------|\n| Family Name                | **Instacart Sans Headline v1.1 SemiBold Italic** | **Instacart Sans Headline v1.1 SemiBold**    |\n| Subfamily Name             | **Regular**                                      | **Italic**                                   |\n| Full Name                  | Instacart Sans Headline v1.1 SemiBold Italic     | Instacart Sans Headline v1.1 SemiBold Italic |\n| Postscript Name            | InstacartSansHeadlinev1.1-SemiBoldItalic         | InstacartSansHeadlinev1.1-SemiBoldItalic     |\n| Typographic Family Name    | Instacart Sans Headline v1.1                     | Instacart Sans Headline v1.1                 |\n| Typographic Subfamily Name | SemiBold Italic                                  | SemiBold Italic                              |",
        severity: "FAIL",
        code: "bad-names",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-SemiBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/font_names",
    check_name: "Check font names are correct",
    check_rationale:
      "Google Fonts has several rules which need to be adhered to when setting a font's name table. Please read: https://googlefonts.github.io/gf-guide/statics.html#supported-styles https://googlefonts.github.io/gf-guide/statics.html#style-linking https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles https://googlefonts.github.io/gf-guide/statics.html#single-weight-families",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "Font names are incorrect:\n\n| Name                       | Current                                           | Expected                                      |\n|----------------------------|---------------------------------------------------|-----------------------------------------------|\n| Family Name                | **Instacart Sans Headline v1.1 ExtraBold Italic** | **Instacart Sans Headline v1.1 ExtraBold**    |\n| Subfamily Name             | **Regular**                                       | **Italic**                                    |\n| Full Name                  | Instacart Sans Headline v1.1 ExtraBold Italic     | Instacart Sans Headline v1.1 ExtraBold Italic |\n| Postscript Name            | InstacartSansHeadlinev1.1-ExtraBoldItalic         | InstacartSansHeadlinev1.1-ExtraBoldItalic     |\n| Typographic Family Name    | Instacart Sans Headline v1.1                      | Instacart Sans Headline v1.1                  |\n| Typographic Subfamily Name | ExtraBold Italic                                  | ExtraBold Italic                              |",
        severity: "FAIL",
        code: "bad-names",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/font_names",
    check_name: "Check font names are correct",
    check_rationale:
      "Google Fonts has several rules which need to be adhered to when setting a font's name table. Please read: https://googlefonts.github.io/gf-guide/statics.html#supported-styles https://googlefonts.github.io/gf-guide/statics.html#style-linking https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles https://googlefonts.github.io/gf-guide/statics.html#single-weight-families",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Regular.ttf",
    section: "Workspace checks",
    subresults: [
      {
        severity: "PASS",
      },
    ],
    worst_status: "PASS",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Regular.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Regular.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Regular.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Regular.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold_Italic.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/font_names",
    check_name: "Check font names are correct",
    check_rationale:
      "Google Fonts has several rules which need to be adhered to when setting a font's name table. Please read: https://googlefonts.github.io/gf-guide/statics.html#supported-styles https://googlefonts.github.io/gf-guide/statics.html#style-linking https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles https://googlefonts.github.io/gf-guide/statics.html#single-weight-families",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        severity: "PASS",
      },
    ],
    worst_status: "PASS",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-Bold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/font_names",
    check_name: "Check font names are correct",
    check_rationale:
      "Google Fonts has several rules which need to be adhered to when setting a font's name table. Please read: https://googlefonts.github.io/gf-guide/statics.html#supported-styles https://googlefonts.github.io/gf-guide/statics.html#style-linking https://googlefonts.github.io/gf-guide/statics.html#unsupported-styles https://googlefonts.github.io/gf-guide/statics.html#single-weight-families",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        severity: "PASS",
      },
    ],
    worst_status: "PASS",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/fstype",
    check_name: "Checking OS/2 fsType does not impose restrictions.",
    check_rationale:
      'The fsType in the OS/2 table is a legacy DRM-related field. Fonts in the Google Fonts collection must have it set to zero (also known as "Installable Embedding"). This setting indicates that the fonts can be embedded in documents and permanently installed by applications on remote systems.\n\nMore detailed info is available at: https://docs.microsoft.com/en-us/typography/opentype/spec/os2#fstype',
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
        severity: "FAIL",
        code: "drm",
        metadata: [
          {
            TableProblem: {
              table_tag: "OS/2",
              field_name: "fsType",
              actual: 256,
              expected: 0,
              message:
                "In this font fsType is set to 256 meaning that:\n* The font may not be subsetted prior to embedding.\n\nNo such DRM restrictions can be enabled on the Google Fonts collection, so the fsType field must be set to zero (Installable Embedding) instead.",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: true,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_glyphs",
    check_name: "Font contains glyphs for whitespace characters?",
    check_rationale:
      "The OpenType specification recommends that fonts should contain glyphs for the following whitespace characters:\n\n- U+0020 SPACE - U+00A0 NO-BREAK SPACE\n\nThe space character is required for text processing, and the no-break space is useful to prevent line breaks at its position. It is also recommended to have a glyph for the tab character (U+0009) and the soft hyphen (U+00AD), but these are not mandatory.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Whitespace glyph missing for codepoint 0x00A0",
        severity: "FAIL",
        code: "missing-whitespace-glyph-0x00A0",
        metadata: [
          {
            GlyphProblem: {
              glyph_name: "uni00A0",
              glyph_id: 0,
              actual: null,
              expected: "U+00A0",
              message: "Whitespace glyph missing for codepoint 0x00A0",
            },
          },
        ],
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "whitespace_widths",
    check_name: "Space and non-breaking space have the same width?",
    check_rationale:
      "If the space and nbspace glyphs have different widths, then Google Workspace has problems with the font.\n\nThe nbspace is used to replace the space character in multiple situations in documents; such as the space before punctuation in languages that do that. It avoids the punctuation to be separated from the last word and go to next line.\n\nThis is automatic substitution by the text editors, not by fonts. It's also used by designers in text composition practice to create nicely shaped paragraphs. If the space and the nbspace are not the same width, it breaks the text composition of documents.",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message: "Space and nbspace not found in font",
        severity: "SKIP",
        code: "missing-glyphs",
      },
    ],
    worst_status: "SKIP",
    hotfix_available: false,
    sourcefix_available: false,
  },
  {
    check_id: "googlefonts/vertical_metrics",
    check_name: "Check font follows the Google Fonts vertical metric schema",
    check_rationale:
      "This check generally enforces Google Fonts’ vertical metrics specifications. In particular: * lineGap must be 0 * Sum of hhea ascender + abs(descender) + linegap must be   between 120% and 200% of UPM * Warning if sum is over 150% of UPM\n\nThe threshold levels 150% (WARN) and 200% (FAIL) are somewhat arbitrarily chosen and may hint at a glaring mistake in the metrics calculations or UPM settings.\n\nOur documentation includes further information: https://github.com/googlefonts/gf-docs/tree/main/VerticalMetrics",
    filename:
      "/Users/simon/Downloads/Workspace PreFixed Fonts/Instacart_Sans_Headline/Instacart_Sans_Headline-ExtraBold.ttf",
    section: "Workspace checks",
    subresults: [
      {
        message:
          "The sum of hhea.ascender + abs(hhea.descender) + hhea.lineGap is 1096 when it should be at least 1200",
        severity: "FAIL",
        code: "bad-hhea-range",
      },
    ],
    worst_status: "FAIL",
    hotfix_available: false,
    sourcefix_available: false,
  },
];
