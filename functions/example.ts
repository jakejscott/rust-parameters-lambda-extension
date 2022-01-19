import { Handler } from "aws-lambda";

export const handler: Handler<String, String> = async (
  name
): Promise<string> => {
  console.log("name", name);
  return `Hello ${name}`;
};
