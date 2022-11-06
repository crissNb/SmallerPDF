# SmallerPDF
Make your PDF files great again! A super simple tool to reduce PDF file size using GhostScript.

# Intention
The main purpose of this program is to reduce the resolution of the PDF, so that you can easily share your large size PDF around online (e.g. Discord has file limit size of 8MB).
This program is especially helpful if you want to reduce the file size of a PDF that contains handwritten notes (e.g. produced with apps like GoodNotes)

# Installation
```bash
cargo install smallerpdf
```

# Usage
From my experience, adjusting lowering image resolution size drastically lowers the output size of the PDF. However, this will lower all the images that are included in the PDF file.
Strangely enough, if you have a PDF that only consists of a handwritten note (e.g. produced with GoodNotes), you may lower the image resolution all the way down to 8 and the quality is still unaffected (but the size is reduced).
```bash
smallerpdf <pdf_name> [image_resolution_size]
```
