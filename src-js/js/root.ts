// INFO: home svg takes user to default dir on click 
//		 upload svg brings up a form in a dialogue
//		 themes svg opens a menu on the svg to pick a theme from 

export async function init() {
	const app = document.createElement("div");
	app.className = "app-root mainApp";

	const nm = await genNavMenu(["home", "upload", "themes", "help"]);
	const fs = await genFS();

	app.append(nm, fs);

	return app;

}

async function genNavMenu(icons: Array<string>) {
	let navMenu = document.createElement("div");
	navMenu.className = "component navMenu";

	icons.forEach(async (icon: string) => navMenu.appendChild(await fetchSVG(icon)))

	return navMenu;
}

async function fetchSVG(icon: string) {
	const res = await fetch(`http://localhost:8765/images/${icon}.svg`, {
		method: "GET",
		headers: {
			"Content-Type": "text/xml+svg",
		},
	});

	const text = await res.text();
	const container = document.createElement("div");
	container.className = "item navMenu-" + icon;
	container.setAttribute("title", icon);

	const svg = new DOMParser().parseFromString(text, "image/svg+xml").querySelector("svg");
	if (svg === null) { throw new Error(`failed to fetch desired svg image file ${icon}.svg`); }

	container.appendChild(svg);

	return container;
}

async function genFS() {
	const container = document.createElement("div");
	container.className = "component fileSystem";
	// TODO: get premade html filled with the requested data from the server

	const res = await fetch("http://localhost:8765/fs?path=/dir/", {
		method: "GET",
		headers: {
			"Content-Type": "text/html",
		}
	});

	const text = await res.text();

	const elems = new DOMParser().parseFromString(text, "text/html").querySelectorAll("div.fsEntry");
	if (elems[0] === undefined) { throw new Error("failed to fetch fs dir"); }

	// @ts-ignore
	container.append(...elems);

	return container;

}

