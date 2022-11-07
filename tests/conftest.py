import os

from hypothesis import Verbosity, settings

settings.register_profile("ci", max_examples=2000)
settings.register_profile("dev", max_examples=100)
settings.register_profile("debug", max_examples=10, verbosity=Verbosity.verbose)
settings.load_profile(os.getenv("HYPOTHESIS_PROFILE", "ci"))
