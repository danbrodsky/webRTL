module VGA where

import Clash.Prelude
import Clash.Sized.Internal.BitVector
import Text.PrettyPrint
import Data.Typeable
import Control.Lens
import Control.Monad
import Control.Monad.Trans.State

-- Plain old Haskell stuff
import Prelude ((+), (-), (*), (==), ($), (.),
    filter, take, fmap, not, error,
    Show,  Bool(True,False), Maybe(Just,Nothing))

data HvSync
  = HvSync
  { _vga_h_sync :: Bool
  , _vga_v_sync :: Bool
  , _inDisplayArea :: Bool
  , _counter_x :: BitVector 10
  , _counter_y :: BitVector 10
  } deriving (Show, Generic, NFDataX)



makeLenses ''HvSync
{-# LANGUAGE RecordWildCards #-}
hvSyncState h@(HvSync {..}) = flip execState h $ do
  if _counter_x == 800 then do
    counter_x .= 0
    if _counter_y == 525 then
      counter_y .= 0
    else
      counter_y .= _counter_y + 1
  else
    counter_x .= _counter_x + 1
  vga_h_sync .= not (_counter_x > (640 + 16) && (_counter_x < (640 + 16 + 96)))
  vga_v_sync .= not (_counter_y > (480 + 10) && (_counter_y < (480 + 10 + 2)))
  inDisplayArea .= ((_counter_x < 640) && (_counter_y < 480))


vga =
  hvSync

  where
    hvSync = register hvSyncInit (hvSyncState <$> hvSync)
    hvSyncInit = HvSync {
      _vga_h_sync = False
    , _vga_v_sync = False
    , _inDisplayArea = False
    , _counter_x = 0
    , _counter_y = 0
    }



getPixel bv ida
  | ida == False = 0 :: BitVector 4
  | otherwise = bv'
    where
      bv' = resize (shiftR (bv .&. 1023) 6)


vgaDemo =
  pixel

  where
    -- TODO: signals should be created in an init function and then passed
    -- to signal creating functions that need their values
    v = vga
    pixel = register pixelInit (getPixel <$> (_counter_x <$> v) <*> (_inDisplayArea <$> v))
    pixelInit = 0 :: BitVector 4




{-# ANN topEntity
  (Synthesize
   { t_name = "vga"
   , t_inputs = [ PortName "clk"
                , PortName "rst"
                , PortName "en" ]
   , t_output = PortName "pixel"
   }) #-}
topEntity
       :: Clock System
       -> Reset System
       -> Enable System
       -> Signal System (BitVector 4)
topEntity = exposeClockResetEnable vgaDemo

