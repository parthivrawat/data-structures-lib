'''Setup configuration for data-structures-lib.'''

from setuptools import setup, find_packages

with open('README.md', 'r', encoding='utf-8') as fh:
    long_description = fh.read()

setup(
    name='data-structures-lib',
    version='1.0.0',
    author='Parthiv Rawat',
    license='MIT',
    description='A comprehensive, zero-dependency collection of fundamental data structures for Python',
    long_description=long_description,
    long_description_content_type='text/markdown',
    url='https://github.com/parthivrawat/data-structures-lib',
    packages=find_packages(),
    package_data={'data_structures_lib': ['py.typed']},
    include_package_data=True,
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'Topic :: Software Development :: Libraries :: Python Modules',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
        'Programming Language :: Python :: 3.12',
    ],
    python_requires='>=3.7',
    install_requires=[],
    extras_require={
        'dev': [
            'pytest>=7.0.0',
            'pytest-cov>=4.0.0',
        ],
    },
)
