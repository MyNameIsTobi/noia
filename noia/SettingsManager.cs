using System;
using System.IO;
using System.Text.Json;
using noia;

public static class SettingsManager
{
    private static readonly string AppFolder = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "noia");
    private static readonly string SettingsFilePath = Path.Combine(AppFolder, "appsettings.json");

    public static AppSettings Settings { get; private set; } = new AppSettings();

    public static void LoadSettings()
    {
        try
        {
            if (File.Exists(SettingsFilePath))
            {
                var json = File.ReadAllText(SettingsFilePath);
                var settings = JsonSerializer.Deserialize<AppSettings>(json);
                if (settings != null)
                {
                    Settings = settings;
                }
            }
        }
        catch
        {
            // Handle errors (logging, etc.) if needed.
            Settings = new AppSettings();
        }
    }

    public static void SaveSettings()
    {
        try
        {
            if (!Directory.Exists(AppFolder))
            {
                Directory.CreateDirectory(AppFolder);
            }
            var json = JsonSerializer.Serialize(Settings, new JsonSerializerOptions { WriteIndented = true });
            File.WriteAllText(SettingsFilePath, json);
        }
        catch
        {
            // Handle errors if needed.
        }
    }
}
