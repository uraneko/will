"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var root_ts_1 = require("./root.ts");
var navMenu = new root_ts_1.NavMenu();
await navMenu.init();
navMenu.start();
var fileManager = new root_ts_1.FileManager();
await fileManager.default();
fileManager.start();
