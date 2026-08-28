'''Core exceptions for the data structures library.'''


class EmptyStructureError(Exception):
    '''Raised when an operation is attempted on an empty structure.'''

    def __init__(self, message: str = 'Structure is empty') -> None:
        super().__init__(message)
