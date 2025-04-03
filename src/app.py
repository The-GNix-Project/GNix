# SPDX-License-Identifier: GPL-3.0-or-later
#
# This file is part of GNix.
#########################################################################################
# GNix - The Graphical Nix Project                                                      #
#---------------------------------------------------------------------------------------#
# GNix is free software: you can redistribute it and/or modify                          #
# it under the terms of the GNU General Public License as published by                  #
# the Free Software Foundation, either version 3 of the License, or any later version.  #
#                                                                                       #
# GNix is distributed in the hope that it will be useful,                               #
# but WITHOUT ANY WARRANTY; without even the implied warranty of                        #
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                         #
# GNU General Public License for more details.                                          #
#                                                                                       #
# You should have received a copy of the GNU General Public License                     #
# along with GNix.  If not, see <https://www.gnu.org/licenses/>.                        #
#########################################################################################
from PyQt5.QtWidgets import QMainWindow, QStackedWidget, QPushButton, QVBoxLayout, QWidget

from src.pages.init_nixos_conf import InitNixosConfig

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