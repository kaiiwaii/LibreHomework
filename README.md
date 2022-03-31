# LibreHomework

LibreHomework is an app made by students and for students. Features
- Adding and removing tasks which are notified by a daemon (only in GNU/Linux)
- Manage your subjects
- Lock your screen to prevent distractions
- Manage your exams (coming soon, 50% done)
- Manage your Documents (coming soon, 50% done)
- Custom network where you can meet other students through their profiles (coming soon, 50% done)
- Available in multiple languages

## Project info
The project is divided in 3 parts:
- [The app/client](https://github.com/HGEpro/LibreHomework/tree/master/app) is written with Rust (backend) and [Tauri](https://github.com/tauri-apps/tauri) + [Svelte](https://github.com/sveltejs/svelte) (frontend). Why did we choose Tauri? Because it's an amazing framework that allows you to write Electron-like desktop apps with [much less resource consumption](https://github.com/tauri-apps/tauri#comparison-between-tauri-and-electron) while still using flexible web technologies. Why Svelte? We all know javascript has lots of frameworks, but Svelte is intuitive and easy to use while at the same time fast as there's no virtual DOM.
- [The daemon](https://github.com/HGEpro/LibreHomework/tree/master/daemon) is written with pure Rust to achieve a low resource consumption. It can be configured by editing the daemonconfig.json in the `.config/LibreHomework` file, but now it needs to be done manually.
- [The server](https://github.com/HGEpro/LibreHomework/tree/master/server) is written in Python with [Sanic](https://github.com/sanic-org/sanic) and postgresql. Why did we choose Python for the server? At first, it was intended to be written with Rust and Rocket.rs, but we finally decided to use python due to security concerns in the SQL library (injections).

## Installation
You can found the compiled releases in the [Releases section](https://github.com/HGEpro/LibreHomework/releases).

In GNU/Linux you can also use a custom installation [script](https://github.com/HGEpro/LibreHomework/tree/main/scripts)_
```
bash <(curl -sL https://raw.githubusercontent.com/HGEpro/LibreHomework/main/scripts/install.sh)
```

## Setup
To run or compile the app from source, you can use follow the steps below:

[Install Tauri and all the dependencies](https://tauri.studio/docs/getting-started/prerequisites)

Run these commands:

```
$ cd LibreHomework/app
$ npm i
```
For running the app, you can use:
```
$ npm run tauri dev
```
And for building the app you can use:
```
$ npm run tauri build
```

For the daemon you can simply type:
```
$ cd LibreHomework/daemon && cargo build --release
```

## Contributing
If you want to contribute to the project, you can fork the project and make your changes or just open a pull request.

## Credits
Thanks to [Wolfram](https://github.com/fabiopolancoe) for the UI code and [YeahNotSewerSide](https://github.com/DoctorEenot) for the technical support.

## Translations
- Russian translation by [YeahNotSewerSide](https://github.com/DoctorEenot)
- Esperanto translation by [Wolfram](https://github.com/fabiopolancoe)
- Portuguese translation by [LDarki](https://github.com/LDarki)
- Polish translation by [Revox](https://github.com/revoxhere)

## Additional info
This project has been created for the [YH4F](https://fsfe.org/activities/yh4f/) 2021-2022.

