# Spartan Key

<u>About:</u> Spartan key is a locally stored password manager that will interface with your chromium based browser.  
It will support multiple vaults and securly encrypt all your sensitive data, not just the passwords.  

<u>How to Install:</u>
1. Clone `Spartan Key` into a directory of your choosing
2. Navigate to the `spartan-key` directory and run the following command to install all needed dependencies: `npm install`
3. Once all dependencies have been installed, you are ready to use `Spartan Key`
4. To run `Spartan Key` run the following command in the `spartan-key` directory: `npm run tauri dev`
5. Running this command will build the application, as well as host the application. 

<u>How to Use: Vault Creation</u>
1. If this is your first time running `Spartan Key` you will be prompted to create a new vault.
2. To create a new vault enter the follwoing information:
    1. Vault Name
    2. Vault Path
    3. Desired Password for the vault
3. The `Vault Path` is needed in order to store the vault, there is a folder icon to the right of the text box, this will allow you to browse to the directory you would like to store the vault in. NOTE: The default location for a vault is `C:\Users\Admin\AppData\Roaming\com.spartankey`, this is also the reccomended location for vault storage. Make sure to type the name of your vault into the `File Name` box in the browse window and click `Save`.
4. Once the vault is created, you will be prompted to enter the password again, once entered, you will now have access to the vault.

<u>How to Use: Vault Enteries</u>
1. Once you have cerated a vault, you will now be able to add your passwords to the vault.
2. To add a password, enter the following under `Add A New Entry`
    1. Entry URL/Name
    2. Username
    3. Password
3. Once you have entered all of this information, click the `Add Entry` button at the bottom.
4. From here, the username and password will be added to the `Vault Entries` on the left side of the application window.

<u>How to Use: Login with Existing Vault</u>
1. Logging into an existing `Spartan Key` vault is simple
2. Once the application is run and a vault is detected, all the user needs to do is enter their password and click `Unlock`

NOTE: By default, the password for the entry will be hidden. In order to show the passwords for the existing entries, click `Show Passwords` in the top left corner. You can also lock the vault be using the `Lock` button, also located in the top left corner.