
module Parallax where


import Clash.Prelude
import Clash.Sized.Internal.BitVector
import Text.PrettyPrint
import Data.Typeable
import Data.Maybe
import Control.Lens
import Control.Monad
import Control.Monad.Trans.State

import Prelude ((+), (-), (*), (==), ($), (.),
    filter, take, fmap, not, error,
    Show,  Bool(True,False), Maybe(Just,Nothing))

data HvSync
  = HvSync
  { _timer :: BitVector 16
  , _counter :: BitVector 16
  , _en :: BitVector 1
  } deriving (Show, Generic, NFDataX)


makeLenses ''HvSync
{-# LANGUAGE RecordWildCards #-}
hvSyncState h@(HvSync {..}) = flip execState h $ do
  if _counter == 3199 then do
    counter .= 0
    timer .= _timer + 1
  else do
    if _en == 0 then
      en .= 1
    else
      counter .= _counter + 1


vga =
  hvSync

  where
    hvSync = register hvSyncInit (hvSyncState <$> hvSync)
    hvSyncInit = HvSync {
      _timer = 0
    , _counter = 0
    , _en = 0
    }

palette i
  | i ==    0   = 0xFF000000
  | i ==    1   = 0xFF101010
  | i ==    2   = 0xFF202020
  | i ==    3   = 0xFF353535
  | i ==    4   = 0xFF454545
  | i ==    5   = 0xFF555555
  | i ==    6   = 0xFF656565
  | i ==    7   = 0xFF757575
  | i ==    8   = 0xFF8A8A8A
  | i ==    9   = 0xFF9A9A9A
  | i ==   10   = 0xFFAAAAAA
  | i ==   11   = 0xFFBABABA
  | i ==   12   = 0xFFCACACA
  | i ==   13   = 0xFFDFDFDF
  | i ==   14   = 0xFFEFEFEF
  | otherwise   = 0xFFFFFFFF :: Unsigned 32

rayCast r =
  hit

  where
    rrrola = 819 :: BitVector 16
    dxax = r `mul` rrrola
    hit = testBit dxax 16

prlx di c =
  color

  where
    r = generate d16 (+di) c
    a = map rayCast r
    color = palette (fromMaybe 15 (elemIndex True a))


prlxDemo =
  pixel

  where
    v = vga
    pixel = register pixelInit (prlx <$> (_counter <$> v) <*> (_timer <$> v))
    pixelInit = 0xFF000000 :: Unsigned 32



{-# ANN topEntity
  (Synthesize
   { t_name = "parallax"
   , t_inputs = [ PortName "clk"
                , PortName "rst"
                , PortName "en" ]
   , t_output = PortName "pixel"
   }) #-}
topEntity
       :: Clock System
       -> Reset System
       -> Enable System
       -> Signal System (Unsigned 32)
topEntity = exposeClockResetEnable prlxDemo
