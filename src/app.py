from PyQt5.QtWidgets import QMainWindow, QStackedWidget, QPushButton, QVBoxLayout, QWidget

import socket

from .pages.init_nixos_conf import InitNixosConfig

class GNix(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("StackedWidget with Separate Widgets")

        # Create a stacked widget
        self.stacked_widget = QStackedWidget(self)
        # self.init_nixos_button = QPushButton("New Nixos Config", self)

        self.model = QVBoxLayout()
        # self.model.addWidget(self.init_nixos_button)
        nav_widget = QWidget()
        nav_widget.setLayout(self.model)
        main_layout = QVBoxLayout()
        main_layout.addWidget(nav_widget)
        main_layout.addWidget(self.stacked_widget)
        central_widget = QWidget()
        central_widget.setLayout(main_layout)
        self.setCentralWidget(central_widget)
        
        self.load_pages()
        self.stacked_widget.setCurrentWidget(self.init_nixos_config)
    
    def load_pages(self):
        self.init_nixos_config = InitNixosConfig()
        self.stacked_widget.addWidget(self.init_nixos_config)