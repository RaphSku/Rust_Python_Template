from .example_project import *


__doc__ = example_project.__doc__
if hasattr(example_project, "__all__"):
    __all__ = example_project.__all__