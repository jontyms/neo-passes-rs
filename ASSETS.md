# Apple Wallet Pass Assets Support

This document describes the asset types supported by the `passes-rs` Python bindings for creating Apple Wallet passes.

## Overview

Apple Wallet passes support various image assets that enhance the visual appearance and functionality of your passes. The `generate_pass()` function now supports all standard Apple Wallet asset types.

## Supported Asset Types

### Icons
- **`icon_path`** - Standard icon (29x29 points)
- **`icon2x_path`** - High-resolution icon (58x58 pixels)

Used when a pass is shown on the lock screen and in apps like Mail when displaying pass attachments.

### Logos
- **`logo_path`** - Standard logo (up to 160x50 points)
- **`logo2x_path`** - High-resolution logo (up to 320x100 pixels)

Displayed in the top-left corner of the pass, next to any logo text. Should typically be narrower than the maximum width.

### Thumbnails
- **`thumbnail_path`** - Standard thumbnail (90x90 points)
- **`thumbnail2x_path`** - High-resolution thumbnail (180x180 pixels)

Displayed next to fields on the front of the pass. Aspect ratio should be 2:3 to 3:2, otherwise the image will be cropped.

### Strip Images
- **`strip_path`** - Standard strip image
- **`strip2x_path`** - High-resolution strip image

Displayed behind primary fields. Dimensions vary by device:
- iPhone 6/6 Plus: 375x98 (event tickets), 375x144 (gift cards/coupons), 375x123 (other)
- Earlier devices: 320x84 (event tickets), 320x110 (square barcode on 3.5" screens), 320x123 (other)

### Background Images
- **`background_path`** - Standard background (180x220 points)
- **`background2x_path`** - High-resolution background (360x440 pixels)

Displayed behind the entire front of the pass. Image is cropped and blurred, so you can often use smaller images that get scaled up.

### Footer Images
- **`footer_path`** - Standard footer (286x15 points)
- **`footer2x_path`** - High-resolution footer (572x30 pixels)

Displayed near the barcode area.

## Usage Example

```python
from passes_rs_py import generate_pass

# Generate pass with all asset types
generate_pass(
    config=pass_json_config,
    cert_path="path/to/cert.pem",
    key_path="path/to/key.key",
    output_path="output.pkpass",
    
    # Icons
    icon_path="assets/icon.png",
    icon2x_path="assets/icon@2x.png",
    
    # Logos
    logo_path="assets/logo.png",
    logo2x_path="assets/logo@2x.png",
    
    # Thumbnails
    thumbnail_path="assets/thumbnail.png",
    thumbnail2x_path="assets/thumbnail@2x.png",
    
    # Strip images
    strip_path="assets/strip.png",
    strip2x_path="assets/strip@2x.png",
    
    # Background images
    background_path="assets/background.png",
    background2x_path="assets/background@2x.png",
    
    # Footer images
    footer_path="assets/footer.png",
    footer2x_path="assets/footer@2x.png",
)
```

## Best Practices

### File Formats
- All images must be in PNG format
- Use RGB color space (not CMYK)
- Avoid transparency in background images

### Resolution Guidelines
- Provide both standard and @2x versions when possible
- @2x images should be exactly double the pixel dimensions
- @3x support is available in the Rust core but not exposed in Python bindings

### Design Recommendations
- **Icons**: Should be simple and recognizable at small sizes
- **Logos**: Keep narrow, use solid colors for best results
- **Thumbnails**: Use square or near-square aspect ratios
- **Backgrounds**: Simple designs work best due to blur effect
- **Strip images**: Consider text legibility over the image

### Optional Parameters
All asset parameters are optional. Only include the ones you need:

```python
# Minimal example with just icons
generate_pass(
    config=config,
    cert_path="cert.pem",
    key_path="key.key", 
    output_path="pass.pkpass",
    icon_path="icon.png"  # Only icon provided
)
```

## Asset Priority by Use Case

### Most Important (recommended for all passes)
1. **Icon** - Required for lock screen display
2. **Logo** - Improves brand recognition

### Highly Recommended
3. **Thumbnail** - Enhances visual appeal
4. **Strip** - Good for passes with primary fields

### Situational
5. **Background** - Use sparingly, can reduce text legibility
6. **Footer** - Useful for branding near barcodes

## Troubleshooting

### Common Issues
- **File not found**: Ensure all provided asset paths exist
- **Wrong format**: Only PNG files are supported
- **Large file sizes**: Optimize images to reduce pass size
- **Poor visibility**: Test text readability over background/strip images

### Error Handling
The function will raise appropriate exceptions for:
- Missing certificate/key files (`PyIOError`)
- Invalid asset files (`PyIOError`) 
- Package creation errors (`PyValueError`)