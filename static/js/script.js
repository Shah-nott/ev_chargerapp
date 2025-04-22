// Updated JavaScript (script.js)
document.addEventListener("DOMContentLoaded", function () {
    // Add event listener for scan button if available
    const scanButton = document.getElementById("scan-button");
    if (scanButton) {
        scanButton.addEventListener("click", scanDevices);
    }

    // Add event listeners for start and stop buttons if available
    const startButton = document.getElementById("start-charging");
    if (startButton) {
        startButton.addEventListener("click", () => startCharging(true));
    }

    const stopButton = document.getElementById("stop-charging");
    if (stopButton) {
        stopButton.addEventListener("click", () => startCharging(false));
    }

    const fetchRecordsButton = document.querySelector("button[onclick='getRecords()']");
    if (fetchRecordsButton) {
        fetchRecordsButton.addEventListener("click", getRecords);
    }

    // Set interval to update charger status every 10 seconds
    setInterval(getChargerStatus, 10000);

    // Initial charger status fetch
    getChargerStatus();
    console.log("JavaScript loaded and DOM is ready!");

    fetch("/api/home")
    .then((res) => {
        if (!res.ok) {
            throw new Error("Not logged in");
        }
        return res.json();
    })
    .then((user) => {
        const userDiv = document.getElementById("user-info");
        userDiv.innerText = `👋 Welcome, ${user.username}`;
    })
    .catch((err) => {
        console.error("User fetch failed", err);
        // Optional: window.location.href = "/login";
    });


});

async function getChargerStatus() {
    try {
        const response = await fetch("/status"); // Corrected URL from `/home/status` to `/status`
        if (response.ok) {
            const status = await response.json();
            console.log("Charger status fetched successfully", status); // Added for debugging

            document.getElementById("device-name").innerText = status.device_name || "N/A";
            document.getElementById("charging-status").innerText = status.charging ? "Yes" : "No";
            document.getElementById("charge-speed").innerText = status.charge_speed || "N/A";
            document.getElementById("amount-spent").innerText = status.amount_spent ? `$${status.amount_spent}` : "N/A";
        } else {
            console.error("Failed to get charger status. Response not ok.");
        }
    } catch (error) {
        console.error("Failed to get charger status:", error);
    }
}

async function scanDevices() {
    console.log("Scan button clicked"); // Debug log to verify if the function is triggered
    try {
        const response = await fetch("/scan");
        if (response.ok) {
            const devices = await response.json();
            console.log("Scanned devices:", devices); // Debug log to check if devices are received

            const deviceList = document.getElementById("device-list");
            deviceList.innerHTML = ""; // Clear previous list

            if (devices.length === 0) {
                console.log("No devices found.");
                const noDeviceItem = document.createElement("li");
                noDeviceItem.textContent = "No devices found.";
                deviceList.appendChild(noDeviceItem);
            }

            devices.forEach(device => {
                console.log("Adding device to list:", device); // Debug log to see each device
                const li = document.createElement("li");
                li.textContent = device;
                li.onclick = () => connectDevice(device);
                deviceList.appendChild(li);
            });
        } else {
            console.error("Failed to scan devices:", response.statusText);
        }
    } catch (error) {
        console.error("Failed to scan devices:", error);
    }
}

async function connectDevice(deviceName) {
    try {
        const response = await fetch("/connect", {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
            },
            body: new URLSearchParams({ sn_number: deviceName }),
        });

        if (response.ok) {
            document.getElementById("connection-status").innerText = `Attempting to connect to ${deviceName}...`;
            checkStatus();
        } else {
            document.getElementById("connection-status").innerText = "Connection attempt failed.";
        }
    } catch (error) {
        console.error("Failed to connect to device:", error);
    }
}

async function checkStatus() {
    // Polling connection status
    setInterval(async () => {
        try {
            const response = await fetch("/status");
            if (response.ok) {
                const status = await response.json();

                if (status.connected) {
                    document.getElementById("connection-status").innerText = `Connected to ${status.device_name}`;
                    document.getElementById("last-connected").innerText = status.last_connected_device;
                } else {
                    document.getElementById("connection-status").innerText = "Not connected.";
                }
            } else {
                console.error("Failed to get connection status. Response not ok.");
            }
        } catch (error) {
            console.error("Failed to get connection status:", error);
        }
    }, 5000);
}

async function getRecords() {
    try {
        const response = await fetch("/records/list");
        if (response.ok) {
            const records = await response.json();
            const recordsList = document.getElementById("records-list");
            recordsList.innerHTML = "";

            records.forEach(record => {
                const div = document.createElement("div");
                div.innerHTML = `
                    <p>Record ID: ${record.record_id}</p>
                    <p>Charger Number: ${record.charger_number}</p>
                    <p>Duration: ${record.duration}</p>
                    <p>Date: ${record.date}</p>
                    <p>Energy Consumed: ${record.energy_consumed} kWh</p>
                    <p>Cost: ${record.cost ? `$${record.cost}` : "Not calculated"}</p>
                    <hr>
                `;
                recordsList.appendChild(div);
            });
        } else {
            console.error("Failed to fetch records:", response.statusText);
        }
    } catch (error) {
        console.error("Failed to fetch records:", error);
    }
}

async function exportToPDF() {
    alert("PDF export feature is coming soon!");
}
