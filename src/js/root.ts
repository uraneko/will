// INFO: home svg takes user to default dir on click 
//		 upload svg brings up a form in a dialogue
//		 themes svg opens a menu on the svg to pick a theme from 

// FIXME: hardcoded server addr is a buggy / uncertain way of doing this
const BaseURI = "192.168.1.104";

export class NavMenu {
	constructor() {
		this.node = document.createElement("div");
		this.node.className = "navMenu";
	}

	static SVG = async function() {
		return {
			home: await this.fetch_svg(BaseURI + "/i/", "home", "text/xml+svg"),
			upload: await this.fetch_svg(BaseURI + "/i/", "upload", "text/xml+svg"),
			themes: await this.fetch_svg(BaseURI + "/i/", "themes", "text/xml+svg"),
		}
	}

	private node: HTMLDivElement;

	async init() {
		const svgs = await NavMenu.SVG();
		for (const svgItem of Object.values(svgs)) { this.node.appendChild(svgItem as HTMLDivElement); }
	}

	// makes a new svg element and adds it to this.node
	// child() {}

	// switches theme
	theme() { }

	// returns to the home dir
	home() { }

	// uploads file(s) 
	upload() { }

	// appends this.node to document body
	start() {
		document.body.appendChild(this.node);
	}

	async fetch_svg(path: string, leaf: string, contentType: string): Promise<HTMLDivElement> {
		const fullPath = path + leaf;
		const res = await fetch(fullPath, {
			method: "GET",
			headers: {
				"Content-Type": contentType,
			}
		});
		const text = res.text();

		const container = document.createElement("div");
		container.className = "NavMenuChild";
		const svg = new DOMParser().parseFromString(text, "image/svg+xml").querySelector("svg");
		if (svg === null) { throw new Error(`fetch_svg(): svg file ${leaf}.svg not found in the provided pathr`); }
		svg.id = leaf;
		container.appendChild(svg);
		return container;
	}
}


export class FileManager {
	constructor() {
		this.node = document.createElement("div");
		this.node.className = "FileManager";
		const container = document.createElement("div");
		container.className = "container";
		this.node.appendChild(container);
	}

	private node: HTMLDivElement;

	// fetches the default dir items
	async default() {
		const res = await fetch(BaseURI + "/d/default", {
			"method": "GET",
			"headers": {
				"Content-Type": "text/html"
			},
		});
		const text = await res.text();
		const html = new DOMParser().parseFromString(text, "text/html").firstElementChild;
		if (html === null) { throw new Error("FileManager.default(): fetched html from server was null"); }

		this.node.firstElementChild!.appendChild(html)
	}

	async fetch_json() {
		const res = fetch("", {});

		return res;
	}

	// adds new file to the filemanager 
	touch() { }

	// cds from dir to another
	async cd(newDir: string) {
		const res = await fetch(BaseURI + "/d/" + newDir, {
			"method": "GET",
			"headers": {
				"Content-Type": "text/html",
			}
		})
	}

	// when requesting a resource, server will first send meta of the resource, 
	// and client would verify before requesting everything
	verifyMeta() { }

	// rms an item in the current dir
	rm() { }

	// adds new dir to file manager
	mkdir() { }

	// lists dir contents
	ls() { }

	// appends the FileManager node to the html document
	start() {
		document.appendChild(this.node);
	}

}
