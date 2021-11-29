// https://stackoverflow.com/questions/26286131/how-do-you-get-the-current-sample-rate-of-windows-audio-playback
// https://git.netflux.io/rob/cpal/src/commit/5cb45bfd7eda2d59bafba38e168c8a5235d30c3d/src/wasapi/device.rs
// https://github.com/brianchung0803/irl_alvr/blob/5849f002428cf712016315f02556444d9f2fa4d8/ALVR-master/alvr/common/src/audio.rs
// https://github.com/HEnquist/wasapi-rs/blob/54802ce52ff0da3f9cb526ffadfe9bce24eb9b9e/src/api.rs

extern crate winapi;

use widestring::*;
use winapi::{
    shared::wtypes::*,
    shared::*,
    um::{
        coml2api::STGM_READ, coml2api::STGM_READWRITE,
        functiondiscoverykeys_devpkey::PKEY_Device_FriendlyName, mmdeviceapi::*,
        objbase::CoInitialize, propidl::*, propsys::IPropertyStore,
    },
    Interface,
};
use std::ptr;
use wio::com::ComPtr;

fn get_device_enumerator() -> *mut IMMDeviceEnumerator {
    let cls_mm_device_enum: guiddef::GUID = CLSID_MMDeviceEnumerator;
    let iid_imm_device_enumerator = IMMDeviceEnumerator::uuidof();

    let mut device_enumerator: *mut IMMDeviceEnumerator = unsafe { std::mem::zeroed() };

    unsafe {
        winapi::um::combaseapi::CoCreateInstance(
            &cls_mm_device_enum,
            std::ptr::null_mut(),
            wtypesbase::CLSCTX_INPROC_SERVER,
            &iid_imm_device_enumerator,
            &mut device_enumerator as *mut *mut IMMDeviceEnumerator
                as *mut *mut winapi::ctypes::c_void,
        );
    }
    return device_enumerator;
}

fn get_imm_device(device_enumerator: *mut IMMDeviceEnumerator) -> *mut IMMDevice {
    let mut pp_device: *mut winapi::um::mmdeviceapi::IMMDevice = unsafe { std::mem::zeroed() };
    unsafe {
        (*device_enumerator).GetDefaultAudioEndpoint(
            winapi::um::mmdeviceapi::eRender,
            winapi::um::mmdeviceapi::eConsole,
            &mut pp_device,
        );
    }
    return pp_device;
}

fn get_device_friendly_name(property_store: ComPtr<IPropertyStore>) -> String {
    let result: String;
    unsafe {
        let mut prop_variant = PROPVARIANT::default();
        let hr = property_store.GetValue(&PKEY_Device_FriendlyName, &mut prop_variant);
        if hr != 0 {
            println!("IPropertyStore::GetValue failed: hr = 0x{:08x}", hr);
        }

        if prop_variant.vt as u32 != VT_LPWSTR {
            println!(
                "PKEY_Device_FriendlyName variant type is {} - expected VT_LPWSTR",
                prop_variant.vt
            );
        }

        result = match U16CStr::from_ptr_str(*prop_variant.data.pwszVal()).to_string() {
            Ok(file) => file,
            Err(e) => {
                println!("U16CStr::from_ptr_str Error {}", e);
                return String::from("");
            }
        };

        let hr = PropVariantClear(&mut prop_variant);
        if hr != 0 {
            println!("PropVariantClear failed: hr = 0x{:08x}", hr);
        }
    }

    result
}

fn get_audio_endpoint_physical_speakers(
    property_store: ComPtr<IPropertyStore>,
    prop_variant: &mut PROPVARIANT,
) -> i32 {
    let result: i32;
    unsafe {
        let hr = property_store.GetValue(&PKEY_AudioEndpoint_PhysicalSpeakers, prop_variant);
        if hr != 0 {
            println!("IPropertyStore::GetValue failed: hr = 0x{:08x}", hr);
        }

        if prop_variant.vt as u32 != VT_UI4 {
            println!(
                "PKEY_AudioEndpoint_PhysicalSpeakers variant type is {} - expected VT_UI4",
                prop_variant.vt
            );
        }

        result = *prop_variant.data.intVal();
    }
    result
}

fn get_audio_endpoint_full_range_speakers(property_store: ComPtr<IPropertyStore>) -> i32 {
    let result: i32;
    unsafe {
        let mut prop_variant = PROPVARIANT::default();
        let hr = property_store.GetValue(&PKEY_AudioEndpoint_FullRangeSpeakers, &mut prop_variant);
        if hr != 0 {
            println!("IPropertyStore::GetValue failed: hr = 0x{:08x}", hr);
        }

        if prop_variant.vt as u32 != VT_UI4 {
            println!(
                "PKEY_AudioEndpoint_FullRangeSpeakers variant type is {} - expected VT_UI4",
                prop_variant.vt
            );
        }

        result = *prop_variant.data.intVal();
        let hr = PropVariantClear(&mut prop_variant);
        if hr != 0 {
            println!("PropVariantClear failed: hr = 0x{:08x}", hr);
        }
    }

    result
}

fn set_audio_endpoint_full_range_speakers(
    property_store: ComPtr<IPropertyStore>,
    physical_speakers: &mut PROPVARIANT,
) {
    unsafe {
        let hr = property_store.SetValue(&PKEY_AudioEndpoint_FullRangeSpeakers, physical_speakers);
        if hr != 0 {
            println!("IPropertyStore::GetValue failed: hr = 0x{:08x}", hr);
        }

        property_store.Commit();
    }
}

fn get_audio_engine_device_format(property_store: ComPtr<IPropertyStore>) {
    unsafe {
        let mut prop_variant = PROPVARIANT::default();
        let hr = property_store.GetValue(&PKEY_AudioEngine_DeviceFormat, &mut prop_variant);
        if hr != 0 {
            println!("IPropertyStore::GetValue failed: hr = 0x{:08x}", hr);
        }

        if prop_variant.vt as u32 != VT_BLOB {
            println!(
                "PKEY_AudioEngine_DeviceFormat variant type is {} - expected VT_BLOB",
                prop_variant.vt
            );
        }

        let result = prop_variant.data.blob_mut().pBlobData as winapi::um::mmsystem::PWAVEFORMATEX;
        let n_channels  = (*result).nChannels ;
        let w_bits_per_sample = (*result).wBitsPerSample;
        let n_samples_per_sec = (*result).nSamplesPerSec;
        let n_avg_bytes_per_sec = (*result).nAvgBytesPerSec;
        println!("Channels: \x1b[0;92m{}\x1b[0m", n_channels);
        if w_bits_per_sample == 16 {
            println!("\x1b[0;93mBits per sample: {} Bit\x1b[0m", w_bits_per_sample);
        } else {
            println!("Bits per sample: \x1b[0;92m{}\x1b[0m Bit", w_bits_per_sample);
        }
        println!("Samples per sec: \x1b[0;92m{}\x1b[0m kHz", n_samples_per_sec/1000);
        println!("Average: \x1b[0;92m{}\x1b[0m bytes/s", n_avg_bytes_per_sec);

        let hr = PropVariantClear(&mut prop_variant);
        if hr != 0 {
            println!("PropVariantClear failed: hr = 0x{:08x}", hr);
        }
    }
}


pub fn apply_audio_settings(write_settings: bool, _default_settings: bool) {
    unsafe { CoInitialize(std::ptr::null_mut()) };
    let device_enumerator: *mut IMMDeviceEnumerator = get_device_enumerator();
    let pp_device: *mut IMMDevice = get_imm_device(device_enumerator);

    unsafe {
        let mut property_store_ptr: *mut IPropertyStore = ptr::null_mut();

        let hr = (*pp_device).OpenPropertyStore(
            if write_settings {
                STGM_READWRITE
            } else {
                STGM_READ
            },
            &mut property_store_ptr as _,
        );
        if hr != 0 {
            println!("IMMDevice::OpenPropertyStore failed: hr = 0x{:08x}", hr);
        }

        let property_store = ComPtr::from_raw(property_store_ptr);
        let friendly_name = get_device_friendly_name(property_store.clone());
        println!("Friendly Name: {}", friendly_name);

        let mut physical_speakers = PROPVARIANT::default();
        let audio_endpoint_physical_speakers =
            get_audio_endpoint_physical_speakers(property_store.clone(), &mut physical_speakers);

        if !write_settings {
            let hr = PropVariantClear(&mut physical_speakers);
            if hr != 0 {
                println!("PropVariantClear failed: hr = 0x{:08x}", hr);
            }
        }

        let audio_endpoint_full_range_speakers =
            get_audio_endpoint_full_range_speakers(property_store.clone());

        if audio_endpoint_physical_speakers == audio_endpoint_full_range_speakers {
            println!("correct setting: Speakers already use the full bandwidth");
        } else {
            if write_settings {
                set_audio_endpoint_full_range_speakers(
                    property_store.clone(),
                    &mut physical_speakers,
                );
                println!("write setting: \x1b[0;92mSpeakers now use the full bandwidth\x1b[0m");
                let hr = PropVariantClear(&mut physical_speakers);
                if hr != 0 {
                    println!("PropVariantClear failed: hr = 0x{:08x}", hr);
                }
            } else {
                println!(
                    "wrong setting: \x1b[0;93mSpeakers do not yet use the full bandwidth\x1b[0m"
                );
            }
        }

        get_audio_engine_device_format(property_store.clone());
    }
}
