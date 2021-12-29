mod bindings;

use block::ConcreteBlock;
use std::ffi::CStr;
use std::convert::TryInto;

use bindings::*;
use neon::event::Channel;
use neon::prelude::*;
use std::sync::Arc;

use fruity::foundation::NSString;

enum MPMediaItemProperty {
    Artist,
    Title,
    AlbumArtist,
    AlbumTitle,
    TrackID,
}

pub struct MediaService {
    info_center: MPNowPlayingInfoCenter,
    playing_info_dict: NSMutableDictionary,
}

const UTF8_ENCODING: NSUInteger = 4;
const ASCII_ENCODING: NSUInteger = 1;

unsafe impl Send for MediaService {} //TODO: Research deletion of that
impl Finalize for MediaService {}

impl MediaService {
    pub fn new(_service_name: String, _identity: String) -> Self {
        let playing_info_dict: NSMutableDictionary;
        let info_center: MPNowPlayingInfoCenter;

        unsafe {
            info_center = MPNowPlayingInfoCenter::defaultCenter();
            info_center.setPlaybackState_(MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped);

            playing_info_dict = NSMutableDictionary(bindings::INSMutableDictionary::<id, id>::init(
                &NSMutableDictionary::alloc(),
            ));
            info_center.setNowPlayingInfo_(NSDictionary(playing_info_dict.0));
        }
        Self {
            info_center,
            playing_info_dict
        }
    }

    fn set_metadata(&self, key: MPMediaItemProperty, value: String)
    {
        unsafe{
            let key = match key {
                MPMediaItemProperty::Artist => MPMediaItemPropertyArtist.0,
                MPMediaItemProperty::Title => MPMediaItemPropertyTitle.0,
                MPMediaItemProperty::AlbumArtist => MPMediaItemPropertyAlbumArtist.0,
                MPMediaItemProperty::AlbumTitle => MPMediaItemPropertyAlbumTitle.0,
                MPMediaItemProperty::TrackID => MPMediaItemPropertyPersistentID.0,
            };
            let str = NSString::from(value.as_str());
            let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
            self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        }
    }

    // region Control
    pub fn is_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_enabled(&self, _enabled: bool) {}
    // endregion Control

    // region Buttons
    pub fn is_play_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_play_enabled(&self, _enabled: bool) {}

    pub fn is_pause_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_pause_enabled(&self, _enabled: bool) {}

    pub fn is_previous_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_previous_enabled(&self, _enabled: bool) {}

    pub fn is_next_enabled(&self) -> bool {
        return false;
    }

    pub fn set_is_next_enabled(&self, _enabled: bool) {}
    // endregion Buttons

    // region Media Information
    pub fn get_media_type(&self) -> i32 {
        return -1;
    }

    pub fn set_media_type(&self, _media_type: i32) {}

    pub fn get_playback_status(&self) -> i32 {
        return -1;
    }

    pub fn set_playback_status(&self, status: i32) {
        let state = match status{
            1 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying,
            2 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateStopped,
            3 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePlaying,
            4 => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStatePaused,
            _ => MPNowPlayingPlaybackState_MPNowPlayingPlaybackStateUnknown,
        };
        
        unsafe {
            self.info_center.setPlaybackState_(state);
        }
    }

    pub fn get_artist(&self) -> String {
        println!("Get artist111");
        let artist: NSString;
        unsafe {
            let info = self.info_center.nowPlayingInfo().0;
            println!("NowPlaying info {:p}", info);
            artist = msg_send!(info, objectForKey: MPMediaItemPropertyArtist.0);
            println!("Artists {}", artist)
        }

        return artist.to_string();
    }

    pub fn set_artist(&self, artist: String) {
        self.set_metadata(MPMediaItemProperty::Artist, artist);
    }

    pub fn get_album_artist(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_artist(&self, album_artist: String) {
        self.set_metadata(MPMediaItemProperty::AlbumArtist, album_artist);
    }

    pub fn get_album_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_album_title(&self, album_title: String) {
        self.set_metadata(MPMediaItemProperty::AlbumTitle, album_title);
    }

    pub fn get_title(&self) -> String {
        return "".to_string();
    }

    pub fn set_title(&self, title: String) {
        self.set_metadata(MPMediaItemProperty::Title, title);
    }

    pub fn get_track_id(&self) -> String {
        return "".to_string();
    }

    pub fn set_track_id(&self, track_id: String) {
        self.set_metadata(MPMediaItemProperty::TrackID, track_id);
    }

    pub fn set_thumbnail(&self, thumbnail_type: i32, thumbnail: String) {
        // match thumbnail_type {
        //     2 => {
        //         let str = NSString::from(value.as_str());
        //         let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
        //         self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        //     },
        //     _ => println!("Unsupported thumbnail type: {}", thumbnail_type),
        // }
        unsafe {
            // let img: objc::runtime::Object = msg_send![NSImage, new];
            let str = "https://i.ytimg.com/vi/7zjPXE-clcU/hq720.jpg?sqp=-oaymwEXCNUGEOADIAQqCwjVARCqCBh4INgESFo&rs=AMzJL3mFsc3BpLft72R0kb8OIkalJddfQA";

            println!("STR");
            let url_str = bindings::NSString::alloc();
            let url_str1 = url_str.initWithBytes_length_encoding_(
                str.as_ptr() as *mut std::ffi::c_void,
                str.len().try_into().unwrap(),
                UTF8_ENCODING,
            );

            
            // let url_str: = bindings::NSString::alloc().initWithBytes_length_encoding_(
            //     str.as_ptr() as *mut std::ffi::c_void,
            //     str.len().try_into().unwrap(),
            //     UTF8_ENCODING,
            // ).unwrap();
            // let url_str_object: bindings::NSObject = *url_str.try_into().unwrap();
            println!("URL321");
            let url = bindings::NSURL::alloc();
            // let url = ;
            println!("URL322");
            // let url = NSURL::URLWithString_(url_str.0);
            let url1: objc::runtime::Object = msg_send!(url, initWithString : url_str1);
            println!("URL333");
            let debug = url.absoluteString();
            println!("URL344");
            println!("URL :{:?}", CStr::from_ptr(debug.cString()));

            println!("IMG1");
            let img = bindings::NSImage::alloc();
            println!("IMG2");
            let _result: objc::runtime::Object = msg_send!(img, initWithContentsOfURL: url1);
            
            println!("ARTWORK");
            let artwork = MPMediaItemArtwork::alloc();
            let _result: objc::runtime::Object = msg_send!(artwork, initWithImage: img);
            
            println!("DICT");
            let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : artwork forKey : MPMediaItemPropertyArtwork.0);
            self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
            // let _result: objc::runtime::Object = msg_send!(img, initWithContentsOfURL: );
            // let str = ;
            // let _result: objc::runtime::Object = msg_send!(self.playing_info_dict, setObject : str forKey : key);
            // self.info_center.setNowPlayingInfo_(NSDictionary(self.playing_info_dict.0));
        }
    }
    // endregion Media Information

    // region Events
    fn send_button_pressed(callback: Arc<Root<JsFunction>>, channel: Channel, button: &'static str) {
        let channel = channel.clone();
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let js_button = cx.string(button);
            let _result = callback.call(&mut cx, this, vec![js_button]);
            Ok(())
        });
    }

    pub fn set_button_pressed_callback(
        &mut self,
        callback: Root<JsFunction>,
        channel: Channel,
    ) -> i64 {
        unsafe {
            let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
            let callback = std::sync::Arc::new(callback);

            let command_handler = ConcreteBlock::new(move |e: MPRemoteCommandEvent| -> MPRemoteCommandHandlerStatus { 
                let command: MPRemoteCommand = msg_send!(e, command);
                let remote_command_center = MPRemoteCommandCenter::sharedCommandCenter();
                let callback = callback.clone();
                let channel = channel.clone();

                if command.0 == remote_command_center.playCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "play");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                if command.0 == remote_command_center.pauseCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "pause");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                if command.0 == remote_command_center.togglePlayPauseCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "playpause");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                if command.0 == remote_command_center.stopCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "stop");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                if command.0 == remote_command_center.nextTrackCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "next");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                if command.0 == remote_command_center.previousTrackCommand().0 {
                    MediaService::send_button_pressed(callback, channel, "previous");
                    return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusSuccess;
                }
                println!("MPRemoteCommand unknown");
                return MPRemoteCommandHandlerStatus_MPRemoteCommandHandlerStatusCommandFailed;
            });
            let command_handler = command_handler.copy();
            remote_command_center.playCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.pauseCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.togglePlayPauseCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.stopCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.nextTrackCommand().addTargetWithHandler_(&*command_handler);
            remote_command_center.previousTrackCommand().addTargetWithHandler_(&*command_handler);
        }
        return -1;
    }

    pub fn remove_button_presed_callback(&mut self) {}
    // endregion Events
}
