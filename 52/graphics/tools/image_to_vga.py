import sys
import requests
import numpy as np
from io import BytesIO
from PIL import Image

ScreenWidth = 80
ScreenHeight = 25

def LoadVgaPaletteFromDumpPpm(DumpPath: str) -> np.ndarray:
    DumpImage = Image.open(DumpPath).convert("RGB")
    DumpArray = np.array(DumpImage)

    ImageWidth = DumpArray.shape[1]
    ImageHeight = DumpArray.shape[0]

    ColumnPixelWidth = ImageWidth // 16
    SampleY = ImageHeight // 2

    Palette = []
    for ColorIndex in range(16):
        SampleX = ColorIndex * ColumnPixelWidth + (ColumnPixelWidth // 2)
        R, G, B = DumpArray[SampleY, SampleX]
        Palette.append([int(R), int(G), int(B)])

    return np.array(Palette, dtype=np.int32)

def DownloadImageAsRgbArray(Url: str) -> np.ndarray:
    Response = requests.get(Url, timeout=30)
    Response.raise_for_status()

    Loaded = Image.open(BytesIO(Response.content)).convert("RGB")
    Resized = Loaded.resize((ScreenWidth, ScreenHeight), Image.Resampling.BILINEAR)

    return np.array(Resized, dtype=np.int32)

def MapPixelsToPaletteIndices(Pixels: np.ndarray, Palette: np.ndarray) -> np.ndarray:
    PixelCount = Pixels.shape[0] * Pixels.shape[1]
    FlatPixels = Pixels.reshape((PixelCount, 3))

    PaletteExpanded = Palette[None, :, :]
    PixelsExpanded = FlatPixels[:, None, :]

    Differences = PixelsExpanded - PaletteExpanded
    Distances = np.sum(Differences * Differences, axis=2)

    Nearest = np.argmin(Distances, axis=1).astype(np.uint8)
    return Nearest

def WriteRustArray(Indices: np.ndarray, OutputPath: str) -> None:
    Values = ", ".join(str(int(x)) for x in Indices.tolist())
    Rust = f"pub const Img: [u8; {len(Indices)}] = [{Values}];\n"

    with open(OutputPath, "w", encoding="utf-8") as File:
        File.write(Rust)

def Main() -> None:
    if len(sys.argv) < 2:
        print("Usage: python3 tools/image_to_vga.py <ImageUrl> [DumpPpmPath]")
        sys.exit(1)

    ImageUrl = sys.argv[1]
    DumpPpmPath = sys.argv[2] if len(sys.argv) >= 3 else "dump.ppm"

    Palette = LoadVgaPaletteFromDumpPpm(DumpPpmPath)
    Pixels = DownloadImageAsRgbArray(ImageUrl)
    Indices = MapPixelsToPaletteIndices(Pixels, Palette)

    OutputPath = "src/colors/img.rs"
    WriteRustArray(Indices, OutputPath)

    print(f"Wrote {OutputPath} ({len(Indices)} cells).")

if __name__ == "__main__":
    Main()
