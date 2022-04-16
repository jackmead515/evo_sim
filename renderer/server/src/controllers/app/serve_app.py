import os
from flask import Blueprint, send_from_directory, send_file

import constants

mod = Blueprint('serve_app', __name__)

@mod.route('/')
def serve_root():
    return send_file(os.path.join(constants.app_dir, 'index.html'))


@mod.route('/static/js/<path:file>')
def serve_js(file):
    js_static_dir = os.path.join(constants.app_dir, 'static/js')
    return send_from_directory(js_static_dir, file)


@mod.route('/static/css/<path:file>')
def serve_css(file):
    css_static_dir = os.path.join(constants.app_dir, 'static/css')
    return send_from_directory(css_static_dir, file)