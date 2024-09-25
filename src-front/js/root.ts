// INFO: home svg takes user to default dir on click 
//		 upload svg brings up a form in a dialogue
//		 themes svg opens a menu on the svg to pick a theme from 

export async function init() {
	const app = document.createElement("div");
	app.className = "app-root mainApp";

	const nm = await genNavMenu(["home", "upload", "themes", "help", "lang", "configs2", "bonfire"]);
	const fe = await genFE();

	app.append(nm, fe);

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

async function genFE() {
	const container = document.createElement("div");
	container.className = "component fileExplorer";

	const path = document.createElement("input");
	path.type = "text";
	path.className = "FEpath";
	path.value = "/dir/";

	container.appendChild(path);

	const res = await fetch("http://localhost:8765/fe?path=/dir/", {
		method: "GET",
		headers: {
			"Content-Type": "text/html",
		}
	});

	const text = await res.text();

	const elems = new DOMParser().parseFromString(text, "text/html").querySelectorAll("div.feEntry");
	if (elems[0] === undefined) { throw new Error("failed to fetch fe dir"); }

	// @ts-ignore
	container.append(...elems);

	return container;

}

async function dir(uri: string) {
	const res = await fetch(uri, {
		method: "GET",
		headers: {
			"Content-Type": "text/html",
		}
	});

	const text = await res.text();

	const elems = new DOMParser().parseFromString(text, "text/html").querySelectorAll("div.feEntry");
	if (elems[0] === undefined) { throw new Error("failed to fetch fe dir"); }

	return elems;
}

function homeClick(e: Event) {
	// TODO: send request to get "/dir"; the landing page default fe path
	// change contents of fileExplorer component
}

function home(home: HTMLDivElement, fn: Function) {
	home.addEventListener("click", homeClick)
}
