from .rsmorphy_lemmatizer import RSMorphyLemmatizer
from sklearn.base import BaseEstimator, TransformerMixin
from typing import List
from ._version import __version__
import os


class RSMorphyTransformer(TransformerMixin, BaseEstimator):
    def __init__(self, dict_path=None, stop_words=(), name_token="[Name]", surname_token="[Surn]",
                 patronymic_token="[Patr]", replace_fio=True, n_jobs=1, split_re="\\s"):
        super().__init__()
        default_dict_path = os.path.join(__path__[0], 'rsmorphy_dict_ru')
        if dict_path is None:
            print("RSMorphyTransformer: Load default dictionary.")
        self.transformer = RSMorphyLemmatizer(default_dict_path if dict_path is None else dict_path,
                                              set(stop_words), name_token, surname_token,
                                              patronymic_token, replace_fio, n_jobs, split_re)
        self.dict_path = dict_path
        self.stop_words = set(stop_words)
        self.name_token = name_token
        self.surname_token = surname_token
        self.patronymic_token = patronymic_token
        self.replace_fio = replace_fio
        self.n_jobs = n_jobs
        self.split_re = split_re

    def fit(self, X, y=None):
        return self

    def to_single_thread(self):
        self.transformer.to_single_thread()
        self.n_jobs = 1

    def transform(self, X: List[str]) -> List[str]:
        return self.transformer.transform(X)

    def __getstate__(self):
        return {
            'dict_path': self.dict_path,
            'stop_words': self.stop_words,
            'name_token': self.name_token,
            'surname_token': self.surname_token,
            'patronymic_token': self.patronymic_token,
            'replace_fio': self.replace_fio,
            'n_jobs': self.n_jobs,
            'split_re': self.split_re
        }

    def __setstate__(self, state):
        self.dict_path = state['dict_path']
        self.stop_words = state['stop_words']
        self.name_token = state['name_token']
        self.surname_token = state['surname_token']
        self.patronymic_token = state['patronymic_token']
        self.replace_fio = state['replace_fio']
        self.n_jobs = state['n_jobs']
        self.split_re = state['split_re']
        if self.dict_path is None or not os.path.exists(self.dict_path):
            print("{} Load default dictionary".format("" if self.dict_path is None else "WARNING: Dictionary path not exists."))
            self.dict_path = os.path.join(__path__[0], 'rsmorphy_dict_ru')
        self.transformer = RSMorphyLemmatizer(self.dict_path, self.stop_words, self.name_token, self.surname_token,
                                              self.patronymic_token, self.replace_fio, self.n_jobs, self.split_re)

