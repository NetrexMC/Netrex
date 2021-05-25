/**
 * ------------------------------------------------------------
 * Script: mod.ts
 * Description: Main script CLI entry point, simple bootstrap
 * Author: @Bavfalcon9
 * ------------------------------------------------------------
 */
import { CommandHandler } from "./commands/Command.ts";
import CountCommand from "./commands/CountCommand.ts";
import HeaderCommand from "./commands/HeaderCommand.ts";

if (import.meta.main) {
	const handler = new CommandHandler();
	handler.register(new CountCommand());
	handler.register(new HeaderCommand());
	handler.runMain();
}