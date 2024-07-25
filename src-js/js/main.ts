import { NavMenu, FileManager, } from "./root.ts";

const navMenu = new NavMenu();
await navMenu.init();
navMenu.start();

const fileManager = new FileManager();
await fileManager.default();
fileManager.start();
