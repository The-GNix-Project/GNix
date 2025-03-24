from .app import GNix
from PyQt5.QtWidgets import QApplication
import sys

def main():
    app = QApplication(sys.argv)
    window = GNix()
    window.show()
    sys.exit(app.exec_())