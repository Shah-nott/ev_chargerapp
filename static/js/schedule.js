document.addEventListener("DOMContentLoaded", function () {
    // Fetch and display schedules when the page loads
    getScheduleList();

    // Add event listener for adding new schedule
    const addScheduleForm = document.getElementById("add-schedule-form");
    if (addScheduleForm) {
        addScheduleForm.addEventListener("submit", async (event) => {
            event.preventDefault();
            addNewSchedule();
        });
    }
});

// Function to fetch and display all schedules
async function getScheduleList() {
    try {
        const response = await fetch("/schedule/list");
        if (response.ok) {
            const schedules = await response.json();
            const scheduleList = document.getElementById("schedule-list");

            scheduleList.innerHTML = ""; // Clear previous list

            schedules.forEach(schedule => {
                const li = document.createElement("li");
                li.textContent = `Charger ID: ${schedule.charger_id}, Start: ${schedule.start_time}, End: ${schedule.end_time}, Recurring: ${schedule.recurring ? 'Yes' : 'No'}`;
                scheduleList.appendChild(li);
            });
        } else {
            console.error("Failed to get schedule list.");
        }
    } catch (error) {
        console.error("Error fetching schedule list:", error);
    }
}

// Function to add a new schedule
async function addNewSchedule() {
    const startTime = document.getElementById("start-time").value;
    const endTime = document.getElementById("end-time").value;
    const recurring = document.getElementById("recurring").checked;

    const scheduleData = {
        charger_id: "123456",  // Replace with dynamic charger ID if available
        start_time: startTime,
        end_time: endTime,
        recurring: recurring
    };

    try {
        const response = await fetch("/schedule/add", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(scheduleData)
        });

        if (response.ok) {
            alert("Schedule added successfully!");
            getScheduleList(); // Refresh the schedule list
        } else {
            alert("Failed to add schedule.");
        }
    } catch (error) {
        console.error("Error adding new schedule:", error);
    }
}
