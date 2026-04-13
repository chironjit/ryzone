/* Statistics Logging */
// Capture stats at the preset update interval and then write to a txt or csv file at the preset logging interval


/* Status and Error Logging */
// Capture any errors, messages, etc from the app into a log file in the .ryzone folder.
// File should be either logs.txt or logs.csv
// Each entry should be timestamped and appended to the bottom of the file and we should store a maximum of 1000 entries.
// If the file is too large, we should delete the oldest entries.
// We should also log the following information:
// - Timestamp
// - Error message
// - Message type (error, warning, info, debug)
// - Additional context (if available)