README
Awami Nastaliq font
===================

Thanks for your interest in Awami Nastaliq. The goal of this “tech-preview” release
is to gain feedback on our approach to using OpenType for smart rendering of Nastaliq.

Our current officially-released font (Awami Nastaliq 3.400: http://software.sil.org/awami)
uses the Graphite system to achieve high-quality layout with behavior characteristic of the
traditional Nastaliq style. The biggest rendering challenges are collision avoidance
and automatic kerning. However, the Graphite system imposes some limitations on the usefulness
of the font, due to a lack of Graphite support in a wide range of applications.

Our current hope is to create a version of Awami Nastaliq that uses OpenType to fix collisions
and perform kerning in a way that properly reflects the Nastaliq tradition. This requires not
only loosening the kerning to avoid collisions between segments, but also tightening the kerning
to create overlaps between diagonal segments, as shown in the images below.

Please go the documentation-techpre/README.md file to see what kinds of feedback we are looking
for.

ABOUT
=====

Awami Nastaliq is a Nastaliq-style Arabic script font supporting a wide variety of
languages of Southwest Asia, including but not limited to Urdu. This font is aimed
at minority language support. This makes it unique among Nastaliq fonts.

Awami means "of the people", "of the common population" or "public".

The Awami Nastaliq font does not provide complete coverage of all the characters
defined in Unicode for Arabic script. Because the font style is specifically
intended for languages using the Nastaliq style of southwest Asia, the character
set for this font is aimed at those languages.

This font makes use of state-of-the-art font technologies to support complex
typographic issues. Font smarts have been implemented using OpenType.

Awami Nastaliq is released under the SIL Open Font License.

Awami is a trademark of SIL Global.

See the OFL and OFL-FAQ for details of the SIL Open Font License.
See the FONTLOG for information on this and previous releases.

For further information about the Graphite version of this font, including Unicode ranges
supported, Graphite font features and how to use them,
and licensing, please see the documentation on the website
(https://software.sil.org/awami) or in the documentation
subfolder of this font package.

TIPS
====

As this font is distributed at no cost, we are unable to provide a
commercial level of personal technical support. The officially released font has, however,
been through some testing on various platforms to be sure it works in most
situations. In particular, it has been tested and shown to work on
Windows 11. Graphite capabilities have been tested on Graphite-supported platforms.

If you do find a problem, please do report it through the website:
https://software.sil.org/awami/support.
We can't guarantee any direct response, but will try to fix reported bugs in
future versions.

Many problems can be solved, or at least explained, through an understanding
of the encoding and use of the fonts. Here are some basic hints:

Encoding:
This font is encoded according to Unicode, so your application must support
Unicode text in order to access letters. Most Windows applications provide
basic Unicode support. For Arabic text, your application must be able to
handle Right to Left text as well as the initial, medial, final forms of each
Arabic letter. You will also need some way of entering Unicode text into your
document.

Rendering:
This font is designed to work with the OpenType advanced font technology.
This technical preview font is primarily intended for testing as the font is
in development.

Keyboarding:
This font package does not include keyboards or other software for entering text.
To type the symbols in this font, use the keyboarding systems provided in your OS
or use a separate utility. Keyman(https://keyman.com/) is a cross-platform keyboarding system.

Various other means may be available for different operating-system platforms to create
additional input methods. Some suggestions are listed here: https://scriptsource.org/entry/ytr8g8n6sw.

CONTACT
========
For more information please visit the Awami Nastaliq page on SIL Global's
Computers and Writing systems website: https://software.sil.org/awami

Support through the website: https://software.sil.org/awami/support
