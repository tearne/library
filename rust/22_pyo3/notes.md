maturin build
pip install --force-reinstall target/wheels/pyinterop-0.1.0-cp39-cp39-manylinux_2_28_x86_64.whl

python
import pyinterop
pyinterop.get_default_user()
user = {"name": "Thor", "email": "thor@asgard.com", "age": 23}
pyinterop.get_contact_info(user)