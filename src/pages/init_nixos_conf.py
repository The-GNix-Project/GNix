from PyQt5.QtWidgets import QWidget, QTreeView, QPushButton, QFileDialog, QLineEdit, QCheckBox, QToolTip
from PyQt5.QtGui import QStandardItemModel, QStandardItem, QIcon
from PyQt5.QtCore import QPoint, QTimer
from PyQt5.uic import loadUi

import socket

HOSTNAME = socket.gethostname()

class InitNixosConfig(QWidget):
    def __init__(self):
        # Type Hints for .ui defined objects
        self.configOverview: QTreeView
        self.newConfigLocation: QPushButton
        self.existingConfigLocation: QPushButton
        self.existingHardwareConfigLocation: QPushButton
        self.newConfig: QLineEdit
        self.existingConfig: QLineEdit
        self.existingHardwareConfig: QLineEdit
        self.enableGit: QCheckBox
        self.enableFlakes: QCheckBox
        self.enableHomeManager: QCheckBox
        self.enableModularConfig: QCheckBox
        self.enableGithub: QCheckBox
        self.enableGitlab: QCheckBox
        self.returnButton: QPushButton
        self.nextButton: QPushButton
        self.block_checkbox_change = False

        super().__init__()
        loadUi("src/pages/Main_Window-New_Nixos_Config.ui", self)
        
        self.model = QStandardItemModel()
        self.configOverview.setModel(self.model)

        rootNode = self.model.invisibleRootItem()

        folderIcon = QIcon('src/assets/icons/folder.png')
        nixIcon = QIcon('src/assets/icons/nix.png')

        hostsFolder = QStandardItem(folderIcon, 'hosts')
        currentHost = QStandardItem(folderIcon, HOSTNAME)
        modulesFolder = QStandardItem(folderIcon, 'modules')
        exampleModule = QStandardItem(folderIcon, 'example_module')
        usersFolder = QStandardItem(folderIcon, 'users')
        exampleUser = QStandardItem(folderIcon, 'example_user')
        userPlaceholder = QStandardItem('user configuration')

        # Add child items to "Folder 1"
        configurationDotNix = QStandardItem(nixIcon, 'configuration.nix')
        HConfigurationDotNix= QStandardItem(nixIcon, 'hardware-configuration.nix')
        defaultDotNix = QStandardItem(nixIcon, 'default.nix')

        currentHost.appendRow([configurationDotNix])
        currentHost.appendRow([HConfigurationDotNix])
        hostsFolder.appendRow([currentHost])

        exampleModule.appendRow([defaultDotNix])
        modulesFolder.appendRow([exampleModule])

        exampleUser.appendRow([userPlaceholder])
        usersFolder.appendRow([exampleUser])

        # Append folders to the root node
        rootNode.appendRows([hostsFolder, modulesFolder, usersFolder, QStandardItem(nixIcon, "flake.nix")])

        self.newConfigLocation.clicked.connect(self.handle_directory(self.newConfig))
        self.existingConfigLocation.clicked.connect(self.handle_file(self.existingConfig))
        self.existingHardwareConfigLocation.clicked.connect(self.handle_file(self.existingHardwareConfig))
        
        self.enableGitlab.stateChanged.connect(self.handle_enable_gitlab)
        self.enableGithub.stateChanged.connect(self.handle_enable_github)
        self.enableGit.stateChanged.connect(self.handle_disable_git)
    
    def handle_file(self, target: QLineEdit, prompt: str = "Open File", initial_path: str = "~/", file_type=None):
        """Returns a function object that will be a handler for opening a file browser that returns a file

        Args:
            target (QLineEdit): path destination
            prompt (str, optional): Prompt to display to user. Defaults to "Open File".
            initial_path (str, optional): First path that opens. Defaults to "~/".
            file_type (Any, optional): File type filter. Defaults to None.
        """
        def handle():
            if handle.file_type is None:
                fname = QFileDialog.getOpenFileName(handle.self, handle.prompt, handle.initial_path)
            else:
                fname = QFileDialog.getOpenFileName(handle.self, handle.prompt, handle.initial_path, handle.file_type)
            handle.target.setText(fname[0])
        handle.prompt = prompt
        handle.initial_path = initial_path
        handle.target = target
        handle.self = self
        handle.file_type = file_type
        return handle

    def handle_directory(self, target: QLineEdit, prompt: str = "Open File", initial_path: str = "~/"):
        """Returns a function object that will be a handler for opening a file browser that returns a directory

        Args:
            target (QLineEdit): path destination
            prompt (str, optional): Prompt to display to user. Defaults to "Open File".
            initial_path (str, optional): First path that opens. Defaults to "~/".
            file_type (Any, optional): File type filter. Defaults to None.
        """
        def handle():
            dirname = QFileDialog.getExistingDirectory(handle.self, handle.prompt, handle.initial_path)
            handle.target.setText(dirname)
        handle.prompt = prompt
        handle.initial_path = initial_path
        handle.target = target
        handle.self = self
        return handle
    
    def handle_enable_gitlab(self, state):
        """handler for checking enableGitlab checkbox

        Args:
            state (int): 0: unchecked, 1: half-checked, 2: checked
        """
        if not state == 2:
            return  
        self.enableGithub.setChecked(False)     
        self.enableGit.setChecked(True)   

    def handle_enable_github(self, state):
        """handler for checking enableGithub checkbox

        Args:
            state (int): 0: unchecked, 1: half-checked, 2: checked
        """
        if not state == 2:
            return  
        self.enableGitlab.setChecked(False)     
        self.enableGit.setChecked(True) 
   
    def handle_disable_git(self, state):
        """handler for unchecking enableGit checkbox

        Args:
            state (int): 0: unchecked, 1: half-checked, 2: checked
        """
        if state == 0 and (self.enableGithub.isChecked() or self.enableGitlab.isChecked()):
            QTimer.singleShot(0, self.defer_enable_git)

            pos = self.enableGit.mapToGlobal(QPoint(0, self.enableGit.height()))
            QToolTip.showText(pos, "You cannot disable Git while GitHub or GitLab is enabled.")

            QTimer.singleShot(3000, QToolTip.hideText)

    def defer_enable_git(self):
        """fix for bug that allowed users to set invalid config
        """
        self.enableGit.setChecked(True)
    
    def handle_next_button(self):
        """This function will generate a configuration for the Nixos configuration directory\n
        example dict structure:
        ```
        {
            "configurationName": "name",
            "configurationLocation": "location",
            "git": True,
            "github": True,
            "gitlab": True,
            "flakes": True,
            "homeManager": True,
            "existingConfig": "location",
            "existingHConfig": "location",
            "directoryTree": [
                {"hosts": {
                    "host1": ["hardware-configuration.nix", "configuration.nix"]
                }},
                {"users": {
                    "user1": []
                }},
                {"modules": {
                    "module1": "default.nix"
                }},
                "flake.nix"
            ]
        }
        ```
        """
        config = {}
        