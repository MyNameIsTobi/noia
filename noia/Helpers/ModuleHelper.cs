using System;
using System.Diagnostics;

namespace noia.Helpers
{
    public static class ModuleHelper
    {
        public static IntPtr GetModuleBaseAddress(Process process, string moduleName)
        {
            foreach (ProcessModule module in process.Modules)
            {
                if (module.ModuleName.Equals(moduleName, StringComparison.OrdinalIgnoreCase))
                {
                    return module.BaseAddress;
                }
            }
            throw new Exception($"Module {moduleName} not found in process {process.ProcessName}.");
        }

        /// <summary>
        /// Retrieves detailed info about a module with the given name from the process.
        /// </summary>
        public static string GetModuleInfo(Process process, string moduleName)
        {
            try
            {
                ProcessModule? foundModule = null;
                foreach (ProcessModule module in process.Modules)
                {
                    if (module.ModuleName.Equals(moduleName, StringComparison.OrdinalIgnoreCase))
                    {
                        foundModule = module;
                        break;
                    }
                }

                if (foundModule != null)
                {
                    var versionInfo = FileVersionInfo.GetVersionInfo(foundModule.FileName);
                    return $"Module: {foundModule.ModuleName}\n" +
                           $"File: {foundModule.FileName}\n" +
                           $"Version: {versionInfo.FileVersion}\n" +
                           $"Memory Size: {foundModule.ModuleMemorySize} bytes";
                }
                else
                {
                    return $"Module '{moduleName}' not found in process '{process.ProcessName}'.";
                }
            }
            catch (Exception ex)
            {
                return $"Error retrieving module info: {ex.Message}";
            }
        }
    }
}
