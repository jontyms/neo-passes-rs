#!/usr/bin/env python3
import os
import sys
from passes_rs_py import PyPassConfig, generate_pass

def main():
            # Paths to your cert & key (put your real files here)
            cert_path = os.path.join("pki", "cert.pem")
            key_path  = os.path.join("pki", "key.key")

            # Asset paths (optional - only include if files exist)
            icon_path = os.path.join("pki", "icon.png")
            icon2x_path = os.path.join("pki", "icon@2x.png")
            logo_path = os.path.join("pki", "logo.png")
            logo2x_path = os.path.join("pki", "logo@2x.png")
            thumbnail_path = os.path.join("pki", "thumbnail.png")
            thumbnail2x_path = os.path.join("pki", "thumbnail@2x.png")
            strip_path = os.path.join("pki", "strip.png")
            strip2x_path = os.path.join("pki", "strip@2x.png")
            background_path = os.path.join("pki", "background.png")
            background2x_path = os.path.join("pki", "background@2x.png")
            footer_path = os.path.join("pki", "footer.png")
            footer2x_path = os.path.join("pki", "footer@2x.png")

            output_path = "/test/example.pkpass"

            # sanity‐check
            if not os.path.isfile(cert_path):
                print(f"ERROR: certificate not found at {cert_path}", file=sys.stderr)
                sys.exit(1)
            if not os.path.isfile(key_path):
                print(f"ERROR: key file not found at {key_path}", file=sys.stderr)
                sys.exit(1)

            # Build a pass config
            config = PyPassConfig(
                organization_name="hackucf.org",
                description="Demo Pass",
                pass_type_identifier="pass.org.example",
                team_identifier="XXXXXXXX",
                serial_number="SN-0002"
            )

            # Generate the .pkpass bundle with all available assets
            try:
                generate_pass(
                    config,
                    cert_path,
                    key_path,
                    output_path,
                    icon_path=icon_path if os.path.isfile(icon_path) else None,
                    icon2x_path=icon2x_path if os.path.isfile(icon2x_path) else None,
                    logo_path=logo_path if os.path.isfile(logo_path) else None,
                    logo2x_path=logo2x_path if os.path.isfile(logo2x_path) else None,
                    thumbnail_path=thumbnail_path if os.path.isfile(thumbnail_path) else None,
                    thumbnail2x_path=thumbnail2x_path if os.path.isfile(thumbnail2x_path) else None,
                    strip_path=strip_path if os.path.isfile(strip_path) else None,
                    strip2x_path=strip2x_path if os.path.isfile(strip2x_path) else None,
                    background_path=background_path if os.path.isfile(background_path) else None,
                    background2x_path=background2x_path if os.path.isfile(background2x_path) else None,
                    footer_path=footer_path if os.path.isfile(footer_path) else None,
                    footer2x_path=footer2x_path if os.path.isfile(footer2x_path) else None,
                )
                print(f"✅ Pass written to {output_path}")
            except Exception as e:
                print("❌ Failed to generate pass:", e, file=sys.stderr)
                sys.exit(1)

if __name__ == "__main__":
            main()
