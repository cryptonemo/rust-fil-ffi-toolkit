use api::errors::SectorManagerErr;
use api::errors::SectorManagerErr::*;
use api::sector_builder::SectorBuilder;
use ffi_toolkit::c_str_to_rust_str;
use libc;
use std::ptr;

// TODO: libfilecoin_proofs.h and libsector_base.h will likely be consumed by
// the same program, so these names need to be unique. Alternatively, figure
// out a way to share this enum across crates in a way that won't cause
// cbindgen to fail.
#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum SBResponseStatus {
    SBNoError = 0,
    SBUnclassifiedError = 1,
    SBCallerError = 2,
    SBReceiverError = 3,
}

pub trait ToResponseStatus {
    fn to_response_status(&self) -> SBResponseStatus;
}

impl<T> ToResponseStatus for Result<T, SectorManagerErr> {
    fn to_response_status(&self) -> SBResponseStatus {
        match self {
            Ok(_) => SBResponseStatus::SBNoError,
            Err(s_m_err) => match s_m_err {
                UnclassifiedError(_) => SBResponseStatus::SBUnclassifiedError,
                CallerError(_) => SBResponseStatus::SBCallerError,
                ReceiverError(_) => SBResponseStatus::SBReceiverError,
            },
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// InitSectorBuilderResponse
/////////////////////////////

#[repr(C)]
pub struct InitSectorBuilderResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_builder: *mut SectorBuilder,
}

impl Default for InitSectorBuilderResponse {
    fn default() -> InitSectorBuilderResponse {
        InitSectorBuilderResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_builder: ptr::null_mut(),
        }
    }
}

impl Drop for InitSectorBuilderResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_init_sector_builder_response(ptr: *mut InitSectorBuilderResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// AddPieceResponse
////////////////////

#[repr(C)]
pub struct AddPieceResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_id: u64,
}

impl Default for AddPieceResponse {
    fn default() -> AddPieceResponse {
        AddPieceResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_id: 0,
        }
    }
}

impl Drop for AddPieceResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_add_piece_response(ptr: *mut AddPieceResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GetMaxStagedBytesPerSector
//////////////////////////////

#[repr(C)]
pub struct GetMaxStagedBytesPerSector {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub max_staged_bytes_per_sector: u64,
}

impl Default for GetMaxStagedBytesPerSector {
    fn default() -> GetMaxStagedBytesPerSector {
        GetMaxStagedBytesPerSector {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            max_staged_bytes_per_sector: 0,
        }
    }
}

impl Drop for GetMaxStagedBytesPerSector {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_get_max_user_bytes_per_staged_sector_response(
    ptr: *mut GetMaxStagedBytesPerSector,
) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// NewSealedSectorAccessResponse
/////////////////////////////////

#[repr(C)]
pub struct NewSealedSectorAccessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_access: *const libc::c_char,
}

impl Default for NewSealedSectorAccessResponse {
    fn default() -> NewSealedSectorAccessResponse {
        NewSealedSectorAccessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_access: ptr::null(),
        }
    }
}

impl Drop for NewSealedSectorAccessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
            drop(c_str_to_rust_str(self.sector_access));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_new_sealed_sector_access_response(
    ptr: *mut NewSealedSectorAccessResponse,
) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// NewStagingSectorAccessResponse
//////////////////////////////////

#[repr(C)]
pub struct NewStagingSectorAccessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_access: *const libc::c_char,
}

impl Default for NewStagingSectorAccessResponse {
    fn default() -> NewStagingSectorAccessResponse {
        NewStagingSectorAccessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_access: ptr::null(),
        }
    }
}

impl Drop for NewStagingSectorAccessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
            drop(c_str_to_rust_str(self.sector_access));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_new_staging_sector_access_response(
    ptr: *mut NewStagingSectorAccessResponse,
) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// WriteAndPreprocesssResponse
///////////////////////////////

#[repr(C)]
pub struct WriteAndPreprocessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes_written: u64,
}

impl Default for WriteAndPreprocessResponse {
    fn default() -> WriteAndPreprocessResponse {
        WriteAndPreprocessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes_written: 0,
        }
    }
}

impl Drop for WriteAndPreprocessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_write_and_preprocess_response(
    ptr: *mut WriteAndPreprocessResponse,
) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// ReadRawResponse
///////////////////

#[repr(C)]
pub struct ReadRawResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub data_len: libc::size_t,
    pub data_ptr: *const u8,
}

impl Default for ReadRawResponse {
    fn default() -> ReadRawResponse {
        ReadRawResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            data_len: 0,
            data_ptr: ptr::null(),
        }
    }
}

impl Drop for ReadRawResponse {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(
                self.data_ptr as *mut u8,
                self.data_len,
                self.data_len,
            ));

            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_read_raw_response(ptr: *mut ReadRawResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// TruncateUnsealedResponse
////////////////////////////

#[repr(C)]
pub struct TruncateUnsealedResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
}

impl Default for TruncateUnsealedResponse {
    fn default() -> TruncateUnsealedResponse {
        TruncateUnsealedResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
        }
    }
}

impl Drop for TruncateUnsealedResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_truncate_unsealed_response(ptr: *mut TruncateUnsealedResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// NumUnsealedBytesResponse
////////////////////////////

#[repr(C)]
pub struct NumUnsealedBytesResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for NumUnsealedBytesResponse {
    fn default() -> NumUnsealedBytesResponse {
        NumUnsealedBytesResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for NumUnsealedBytesResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_num_unsealed_bytes_response(ptr: *mut NumUnsealedBytesResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GetMaxUserBytesPerStagedSectorResponse
//////////////////////////////////////////

#[repr(C)]
pub struct GetMaxUserBytesPerStagedSectorResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for GetMaxUserBytesPerStagedSectorResponse {
    fn default() -> GetMaxUserBytesPerStagedSectorResponse {
        GetMaxUserBytesPerStagedSectorResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for GetMaxUserBytesPerStagedSectorResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_get_max_user_bytes_per_staged_sector(
    ptr: *mut GetMaxUserBytesPerStagedSectorResponse,
) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// MaxUnsealedBytesPerSectorResponse
/////////////////////////////////////

#[repr(C)]
pub struct MaxUnsealedBytesPerSectorResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for MaxUnsealedBytesPerSectorResponse {
    fn default() -> MaxUnsealedBytesPerSectorResponse {
        MaxUnsealedBytesPerSectorResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for MaxUnsealedBytesPerSectorResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_max_unsealed_bytes_per_sector_response(
    ptr: *mut MaxUnsealedBytesPerSectorResponse,
) {
    let _ = Box::from_raw(ptr);
}
