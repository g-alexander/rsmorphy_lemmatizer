# RSMorphy Lemmatizer

This is simple package for lemmatize (word normal form) russian words.
Package based on rsmorphy: https://github.com/g-alexander/rsmorphy

## Installation
```bash
pip install rsmorphy_lemmatizer --upgrade
```

## Usage
```python
from rsmorphy_lemmatizer import RSMorphyTransformer

transformer = RSMorphyTransformer()
test_strings = [
    "Мама мыла раму"
]
print(transformer.transform(test_strings))

# Output: ['мама мыло рам']
```

for multithread transform:
```python
transformer = RSMorphyTransformer(n_jobs=3)
```

to back to single thread:
```python
transformer.to_single_thread()
```