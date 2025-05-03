#!/usr/bin/env python3
import os
import sys
from passes_rs_py import PyPassConfig, generate_pass

def main():
            # Paths to your cert & key (put your real files here)
            cert_path = os.path.join("pki", "cert.pem")
            key_path  = os.path.join("pki", "key.key")
            icon_path = os.path.join("pki", "icon.png")
            icon2x_path = os.path.join("pki", "icon@2x.png")
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

            # Generate the .pkpass bundle
            try:
                generate_pass(config, cert_path, key_path, output_path, icon_path, icon2x_path)
                print(f"✅ Pass written to {output_path}")
            except Exception as e:
                print("❌ Failed to generate pass:", e, file=sys.stderr)
                sys.exit(1)

if __name__ == "__main__":
            main()
