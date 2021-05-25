export default abstract class Command {
	/**
	 * The label of the command.
	 * This is NOT the name. EG: "help.info.command"
	 */
	public abstract label: string;

	/**
	 * The name of the command.
	 */
	public abstract name: string;

	/**
	 * A list of possible aliases for this command.
	 */
	public abstract aliases: string[];

	/**
	 * Any description for the command.
	 */
	public abstract description: string;


	/**
	 * A example on how to use the command.
	 * EG: hellworld <foo> [bar?]
	 */
	public abstract usage: string;

	/**
	 * Executes a command
	 */
	public abstract execute(...args: any[]): any;
}

export class DefaultHelp extends Command {
	public label: string = "command.help";
	public name: string = "help";
	public aliases: string[] = ["h"];
	public description: string = "Shows help menu.";
	public usage: string = "{c} [command]";

	public execute(command: string) {
		const cmdh = CommandHandler.instance;
		const padding: number = cmdh.commands.map((v) => (v.name).length).reduce((v, c) => {
			return (Math.abs(c - v)) < Math.abs(v) ? c : v;
		});
		const padMessage = (name: string, description: string) => {
			return name +
				" ".repeat(padding - name.length > 0 ? padding - name.length : 0) +
				" - " + description;
		}

		if (!command) {
			// show help
			const msgs: string[] = cmdh.commands.map(m => " " + padMessage(m.name, m.description));
			console.log("COMMANDS:\n" + msgs.join("\n"));
		} else {
			if (!cmdh.getCommand(command)) {
				console.log("Unknown command! Try \"help\" for a list of commands.");
			} else {
				const cmd = cmdh.getCommand(command);
				console.log("ALIASES:\n " + cmd?.aliases.join("\n "));
				console.log("\nUSAGE:\n " + cmd?.usage.replace(/\{c\}/igm, command));
				console.log("\nDESCRIPTION:\n " + cmd?.description);
			}
		}
	}
}

export class CommandHandler {
	public commands: Command[];
	public static instance: CommandHandler;

	public constructor() {
		this.commands = [
			new DefaultHelp()
		];
		CommandHandler.instance = this;
	}

	public register(cmd: Command): void {
		if (this.hasCommand(cmd.name)) {
			throw new Error(`Command with label "${cmd.label}" already registered.`);
		}
		this.commands.push(cmd);
	}

	public unregister(label: string): void {
		if (!this.hasCommand(label)) {
			throw new Error(`No command found with that label or name.`);
		}
	}

	public hasCommand(labelOrName: string): boolean {
		return !!this.commands.find((cmd) => cmd.label === labelOrName || cmd.name === labelOrName);
	}

	public getCommand(aliasLableOrName: string): Command | undefined {
		return this.commands.find((cmd) =>
			cmd.label === aliasLableOrName ||
			cmd.name === aliasLableOrName ||
			cmd.aliases.includes(aliasLableOrName)
		);
	}

	public runMain(): void {
		const command = Deno.args[0];
		if (!command) {
			return this.getCommand("help")?.execute();
		} else {
			if (!this.getCommand(command)) {
				return this.getCommand("help")?.execute(command);
			}
			return this.getCommand(command)?.execute(...(Deno.args.slice(1) ?? []));
		}
	}
}