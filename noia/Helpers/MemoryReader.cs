using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace noia.Helpers
{
    // A structure to hold the memory values.
    public struct MemoryValues
    {
        public int HP;
        public int HPMax;
        public int Mana;
        public int ManaMax;
    }

    public static class MemoryReader
    {
        [DllImport("kernel32.dll", SetLastError = true)]
        private static extern bool ReadProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress,
            [Out] byte[] lpBuffer, int dwSize, out int lpNumberOfBytesRead);

        /// <summary>
        /// Reads a 32-bit integer from the target process at the specified address.
        /// </summary>
        public static int ReadInt32(Process process, IntPtr address)
        {
            int bytesRead;
            byte[] buffer = new byte[4];
            bool success = ReadProcessMemory(process.Handle, address, buffer, buffer.Length, out bytesRead);
            if (!success || bytesRead != buffer.Length)
            {
                throw new Exception("Failed to read memory at " + address.ToString("X"));
            }
            return BitConverter.ToInt32(buffer, 0);
        }

        /// <summary>
        /// Reads memory values (HP, HPMax, Mana, ManaMax) from the target process.
        /// It calculates the actual addresses by adding offsets (from ModuleAddresses)
        /// to the base address of the specified module (e.g., "game.dll").
        /// </summary>
        public static MemoryValues ReadMemoryValues(Process process, string moduleName)
        {
            // Get the base address of the module (e.g., "game.dll").
            IntPtr baseAddress = ModuleHelper.GetModuleBaseAddress(process, moduleName);

            // Calculate the actual addresses by adding the offsets.
            IntPtr hpAddress = IntPtr.Add(baseAddress, ModuleAddresses.HP.ToInt32());
            IntPtr hpMaxAddress = IntPtr.Add(baseAddress, ModuleAddresses.HPMax.ToInt32());
            IntPtr manaAddress = IntPtr.Add(baseAddress, ModuleAddresses.Mana.ToInt32());
            IntPtr manaMaxAddress = IntPtr.Add(baseAddress, ModuleAddresses.ManaMax.ToInt32());

            MemoryValues values = new MemoryValues
            {
                HP = ReadInt32(process, hpAddress),
                HPMax = ReadInt32(process, hpMaxAddress),
                Mana = ReadInt32(process, manaAddress),
                ManaMax = ReadInt32(process, manaMaxAddress)
            };

            return values;
        }
    }
}
