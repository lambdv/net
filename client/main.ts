import net from "node:net";
const data = {
  x: 1,
  y: 2,
};

const s = net.createConnection({ port: 3000, host: "127.0.0.1" }, () => {
  // Connected
  s.write(JSON.stringify(data));
  s.end(); // Properly close the connection
});

s.on("data", (d: any) => {
  let rest = fetch("test");
  console.log("Received:", d.toString());
});

// s.on("end", () => {
//   console.log("Disconnected from server");
// });

// s.on("error", (err) => {
//   console.error("Socket error:", err);
// });
