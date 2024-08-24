package fr.myapplication.adapter;

import android.net.Uri
import android.view.LayoutInflater
import android.view.View;
import android.view.ViewGroup
import android.widget.ImageView;
import android.widget.TextView
import fr.myapplication.R;

import androidx.recyclerview.widget.RecyclerView;
import com.bumptech.glide.Glide
import fr.myapplication.MainActivity
import fr.myapplication.ScanModel
import fr.myapplication.ScanPopup
import fr.myapplication.ScanUpdate
import java.io.File

class ScanAdapter(
    val context: MainActivity,
    private val scanList: List<ScanModel>,
    private val layoutId: Int
) : RecyclerView.Adapter<ScanAdapter.ViewHolder>() {

    class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        val scanImage = view.findViewById<ImageView>(R.id.image_item)
        val scanName: TextView? = view.findViewById(R.id.name_item)
        val scanNb_ep: TextView? = view.findViewById(R.id.nb_episode_item)
        val starIcon: ImageView? = view.findViewById(R.id.start_item)
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val view = LayoutInflater
            .from(parent.context)
            .inflate(layoutId, parent, false)
        return ViewHolder(view)
    }

    override fun getItemCount(): Int = scanList.size

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        //recup info scan
        val currentScan = scanList[position]

        //charger image
        Glide.with(context).load(Uri.parse(currentScan.imageFile)).into(holder.scanImage)

        //charge nom
        holder.scanName?.text = currentScan.name

        //charge nombre
        holder.scanNb_ep?.text = currentScan.nb_episode.toString().plus(" ").plus("episode")

        //verif si like
        if (currentScan.liked) {
            holder.starIcon?.setImageResource(R.drawable.ic_star)
        } else {
            holder.starIcon?.setImageResource(R.drawable.ic_unstar)
        }

        holder.starIcon?.setOnClickListener {
            currentScan.liked = !currentScan.liked
            if (currentScan.liked) {
                holder.starIcon.setImageResource(R.drawable.ic_star)
            } else {
                holder.starIcon.setImageResource(R.drawable.ic_unstar)
            }
            ScanUpdate(this, currentScan).updateStarButton()
        }

        holder.scanImage.setOnClickListener {
            ScanPopup(this, currentScan).show()
        }
    }
}