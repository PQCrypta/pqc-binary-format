#!/usr/bin/env python3
"""
Setup script for PQC Binary Format Python bindings
"""

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pqc-binary-format",
    version="1.0.5",
    description="Standardized binary format for post-quantum cryptography encrypted data interchange",
    long_description=open("../../README.md").read(),
    long_description_content_type="text/markdown",
    author="Allan",
    author_email="allan@pqcrypta.com",
    url="https://github.com/PQCrypta/pqcrypta-community",
    license="MIT OR Apache-2.0",
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "License :: OSI Approved :: Apache Software License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Security :: Cryptography",
        "Topic :: Software Development :: Libraries",
    ],
    keywords="cryptography post-quantum pqc encryption binary-format",
    python_requires=">=3.8",
    rust_extensions=[
        RustExtension(
            "pqc_binary_format",
            path="../../Cargo.toml",
            binding=Binding.PyO3,
            features=["python"],
        )
    ],
    zip_safe=False,
)
