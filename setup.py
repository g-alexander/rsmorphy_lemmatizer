from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rsmorphy_lemmatizer",
    version="0.1.3",
    rust_extensions=[RustExtension("rsmorphy_lemmatizer.rsmorphy_lemmatizer", binding=Binding.PyO3)],
    packages=["rsmorphy_lemmatizer"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    install_requires=['scikit-learn', 'setuptools-rust']
)