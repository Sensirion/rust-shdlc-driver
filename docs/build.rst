Building & Installation
=======================

Requirements
------------

- **Rust**: Version >= 1.75 with ``cargo``
- **Python**: Version >= 3.11 with ``pip``
- **Maturin**: Version >= 1.5 (``pip install maturin``)

Setting Up a Python Virtual Environment
---------------------------------------

.. note::
   **Recommendation:** It is strongly recommended to install and run ``rust-shdlc-driver`` within a dedicated Python virtual environment (``venv``). This ensures dependencies remain isolated and prevents conflicts with system Python packages.

Create and activate a virtual environment using Python 3.11+:

**Linux / macOS:**

.. code-block:: bash

   # Create virtual environment
   python3.11 -m venv .venv

   # Activate virtual environment
   source .venv/bin/activate

   # Upgrade pip and install build tool
   pip install --upgrade pip maturin

**Windows:**

.. code-block:: bat

   # Create virtual environment
   py -3.11 -m venv .venv

   # Activate virtual environment
   .venv\Scripts\activate

   # Upgrade pip and install build tool
   pip install --upgrade pip maturin

Building the Python Package with Maturin
----------------------------------------

Once the virtual environment is activated, build and install the driver:

.. code-block:: bash

   # Development build (editable install with debug info)
   maturin develop

   # Release build (optimized native performance)
   maturin develop --release

Building Wheels for Distribution
--------------------------------

To build universal ABI3 wheels compatible with any Python >= 3.11:

.. code-block:: bash

   maturin build --release

The resulting ``.whl`` files will be placed into the ``target/wheels/`` directory and can be distributed or installed with ``pip install target/wheels/*.whl``.

Running Tests
-------------

.. code-block:: bash

   # Run Rust tests
   cargo test

   # Install test dependencies in venv
   pip install pytest pytest-asyncio

   # Run Python pytest suite
   pytest python_tests/ -v
