# -*- coding: utf-8 -*-
project = 'rust-shdlc-driver'
copyright = '2026, Sensirion AG'
author = 'Sensirion AG'
version = '0.1.0'
release = '0.1.0'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon',
    'sphinx.ext.viewcode',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

html_theme = 'sphinx_rtd_theme'
html_static_path = []
html_title = "rust-shdlc-driver Documentation"
