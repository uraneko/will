"use strict";
// INFO: home svg takes user to default dir on click 
//		 upload svg brings up a form in a dialogue
//		 themes svg opens a menu on the svg to pick a theme from 
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.FileManager = exports.NavMenu = void 0;
// FIXME: hardcoded server addr is a buggy / uncertain way of doing this
var BaseURI = "192.168.1.104";
var NavMenu = /** @class */ (function () {
    function NavMenu() {
        this.node = document.createElement("div");
        this.node.className = "navMenu";
    }
    NavMenu.prototype.init = function () {
        return __awaiter(this, void 0, void 0, function () {
            var svgs, _i, _a, svgItem;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0: return [4 /*yield*/, NavMenu.SVG()];
                    case 1:
                        svgs = _b.sent();
                        for (_i = 0, _a = Object.values(svgs); _i < _a.length; _i++) {
                            svgItem = _a[_i];
                            this.node.appendChild(svgItem);
                        }
                        return [2 /*return*/];
                }
            });
        });
    };
    // makes a new svg element and adds it to this.node
    // child() {}
    // switches theme
    NavMenu.prototype.theme = function () { };
    // returns to the home dir
    NavMenu.prototype.home = function () { };
    // uploads file(s) 
    NavMenu.prototype.upload = function () { };
    // appends this.node to document body
    NavMenu.prototype.start = function () {
        document.body.appendChild(this.node);
    };
    NavMenu.prototype.fetch_svg = function (path, leaf, contentType) {
        return __awaiter(this, void 0, void 0, function () {
            var fullPath, res, text, container, svg;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        fullPath = path + leaf;
                        return [4 /*yield*/, fetch(fullPath, {
                                method: "GET",
                                headers: {
                                    "Content-Type": contentType,
                                }
                            })];
                    case 1:
                        res = _a.sent();
                        text = res.text();
                        container = document.createElement("div");
                        container.className = "NavMenuChild";
                        svg = new DOMParser().parseFromString(text, "image/svg+xml").querySelector("svg");
                        if (svg === null) {
                            throw new Error("fetch_svg(): svg file ".concat(leaf, ".svg not found in the provided pathr"));
                        }
                        svg.id = leaf;
                        container.appendChild(svg);
                        return [2 /*return*/, container];
                }
            });
        });
    };
    NavMenu.SVG = function () {
        return __awaiter(this, void 0, void 0, function () {
            var _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        _a = {};
                        return [4 /*yield*/, this.fetch_svg(BaseURI + "/i/", "home", "text/xml+svg")];
                    case 1:
                        _a.home = _b.sent();
                        return [4 /*yield*/, this.fetch_svg(BaseURI + "/i/", "upload", "text/xml+svg")];
                    case 2:
                        _a.upload = _b.sent();
                        return [4 /*yield*/, this.fetch_svg(BaseURI + "/i/", "themes", "text/xml+svg")];
                    case 3: return [2 /*return*/, (_a.themes = _b.sent(),
                            _a)];
                }
            });
        });
    };
    return NavMenu;
}());
exports.NavMenu = NavMenu;
var FileManager = /** @class */ (function () {
    function FileManager() {
        this.node = document.createElement("div");
        this.node.className = "FileManager";
        var container = document.createElement("div");
        container.className = "container";
        this.node.appendChild(container);
    }
    // fetches the default dir items
    FileManager.prototype.default = function () {
        return __awaiter(this, void 0, void 0, function () {
            var res, text, html;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, fetch(BaseURI + "/d/default", {
                            "method": "GET",
                            "headers": {
                                "Content-Type": "text/html"
                            },
                        })];
                    case 1:
                        res = _a.sent();
                        return [4 /*yield*/, res.text()];
                    case 2:
                        text = _a.sent();
                        html = new DOMParser().parseFromString(text, "text/html").firstElementChild;
                        if (html === null) {
                            throw new Error("FileManager.default(): fetched html from server was null");
                        }
                        this.node.firstElementChild.appendChild(html);
                        return [2 /*return*/];
                }
            });
        });
    };
    FileManager.prototype.fetch_json = function () {
        return __awaiter(this, void 0, void 0, function () {
            var res;
            return __generator(this, function (_a) {
                res = fetch("", {});
                return [2 /*return*/, res];
            });
        });
    };
    // adds new file to the filemanager 
    FileManager.prototype.touch = function () { };
    // cds from dir to another
    FileManager.prototype.cd = function (newDir) {
        return __awaiter(this, void 0, void 0, function () {
            var res;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, fetch(BaseURI + "/d/" + newDir, {
                            "method": "GET",
                            "headers": {
                                "Content-Type": "text/html",
                            }
                        })];
                    case 1:
                        res = _a.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    // when requesting a resource, server will first send meta of the resource, 
    // and client would verify before requesting everything
    FileManager.prototype.verifyMeta = function () { };
    // rms an item in the current dir
    FileManager.prototype.rm = function () { };
    // adds new dir to file manager
    FileManager.prototype.mkdir = function () { };
    // lists dir contents
    FileManager.prototype.ls = function () { };
    // appends the FileManager node to the html document
    FileManager.prototype.start = function () {
        document.appendChild(this.node);
    };
    return FileManager;
}());
exports.FileManager = FileManager;
