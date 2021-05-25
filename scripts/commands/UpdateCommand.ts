import Command from "./Command.ts";

export default class UpdateCommand extends Command {
	public label: string = "command.update";
	public name: string = "update";
	public aliases: string[] = [
		"upgrade",
		"install"
	];
	public description: string = "Updates netrexscript.";
	public usage: string = "{c}";

	public async execute() {
		console.log("Upgrading to latest...");
		const cache = Deno.run({
				cmd: ("deno cache --reload https://raw.githubusercontent.com/NetrexMC/Netrex-deno/master/scripts/mod.ts").split(" "),
				stdout: "piped"
		});

		await cache.output();

		if (!(await cache.status()).success) {
			console.log("%cCache failed to reload. Proceeding anyway.", "color: #fce262");
		}

		const install = Deno.run({
			cmd: `deno install -A -f -n netrexscript https://raw.githubusercontent.com/NetrexMC/Netrex-deno/master/scripts/mod.ts`.split(" "),
			stdout: "piped"
		})

		await install.output();

		if (!(await install.status()).success) {
			console.log("%cFailed to upgrade! Try again and see if the problem persists.", "color: #f44949");
		} else {
			console.log("%cUpgraded successfully!", "color: #19ea3c");
		}
	}
}