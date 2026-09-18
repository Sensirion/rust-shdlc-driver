Developer Guide & Build Instructions
====================================

This guide describes how to build, test, and package **rust-shdlc-driver** from source for both Rust and Python development.

Prerequisites & Toolchain
-------------------------

- **Rust**: Version >= 1.75 with ``cargo``
- **Python**: Version >= 3.11 with ``pip`` and ``venv``
- **Maturin**: Version >= 1.5 (``pip install maturin``)

Building the Rust Package
-------------------------

The core library is written in Rust and can be built using standard ``cargo`` commands:

.. code-block:: bash

   # Debug build
   cargo build

   # Release build (optimized)
   cargo build --release

   # Check examples
   cargo check --examples

   # Run a specific example
   cargo run --example mock_device
   cargo run --example sync_device
   cargo run --example async_device

Building the Python Package
---------------------------

Setting Up a Python Virtual Environment
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

It is strongly recommended to use a dedicated Python virtual environment for isolated dependency management:

**Linux / macOS:**

.. code-block:: bash

   # Create a virtual environment using Python 3.11+
   python3.11 -m venv .venv

   # Activate the virtual environment
   source .venv/bin/activate

   # Upgrade pip and install build tools
   pip install --upgrade pip maturin

**Windows:**

.. code-block:: bat

   # Create a virtual environment using Python 3.11+
   py -3.11 -m venv .venv

   # Activate the virtual environment
   .venv\Scripts\activate

   # Upgrade pip and install build tools
   pip install --upgrade pip maturin

Building and Installing with Maturin
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Once your virtual environment is active:

.. code-block:: bash

   # Development build (editable install with debug symbols)
   maturin develop

   # Release build (optimized native performance)
   maturin develop --release

Building Wheels for Distribution
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

To build universal ABI3 binary wheels compatible with any Python >= 3.11:

.. code-block:: bash

   maturin build --release

The resulting ``.whl`` packages will be placed into the ``target/wheels/`` directory and can be installed via ``pip install target/wheels/*.whl``.

Running Tests
-------------

.. code-block:: bash

   # Run Rust unit and integration tests
   cargo test

   # Install test dependencies in venv
   pip install pytest pytest-asyncio

   # Run Python pytest suite
   pytest python_tests/ -v

Building Documentation
----------------------

The documentation is built using Sphinx:

.. code-block:: bash

   # Install documentation dependencies
   pip install sphinx sphinx_rtd_theme

   # Build HTML documentation
   sphinx-build -b html docs docs/_build/html
